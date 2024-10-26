use crate::{
    message::{MessageContext, MessageContextParcel},
    protoburn::{Protoburn, Protoburns},
};
use anyhow::Result;
use bitcoin::{Block, Transaction, Txid};
use metashrew::index_pointer::{AtomicPointer, IndexPointer};
use ordinals::Runestone;
use protorune_support::{
    balance_sheet::BalanceSheet,
    protostone::{split_bytes, Protostone},
    rune_transfer::{OutgoingRunes, RuneTransfer},
    utils::encode_varint_list,
};
use std::collections::{HashMap, HashSet};
static mut PROTOCOLS: Option<HashSet<u128>> = None;

pub fn initialized_protocol_index() -> Result<()> {
    unsafe { PROTOCOLS = Some(HashSet::new()) }
    Ok(())
}

pub fn add_to_indexable_protocols(protocol_tag: u128) -> Result<()> {
    unsafe {
        if let Some(set) = PROTOCOLS.as_mut() {
            set.insert(protocol_tag);
        }
    }
    Ok(())
}

pub trait MessageProcessor {
    fn process_message<T: MessageContext>(
        &self,
        atomic: &mut AtomicPointer,
        transaction: &Transaction,
        txindex: u32,
        block: &Block,
        height: u64,
        _runestone_output_index: u32,
        vout: u32,
        balances_by_output: &mut HashMap<u32, BalanceSheet>,
        default_output: u32,
    ) -> Result<()>;
}

impl MessageProcessor for Protostone {
    fn process_message<T: MessageContext>(
        &self,
        atomic: &mut AtomicPointer,
        transaction: &Transaction,
        txindex: u32,
        block: &Block,
        height: u64,
        _runestone_output_index: u32,
        vout: u32,
        balances_by_output: &mut HashMap<u32, BalanceSheet>,
        default_output: u32,
    ) -> Result<()> {
        if self.is_message() {
            let initial_sheet = balances_by_output
                .get(&vout)
                .map(|v| v.clone())
                .unwrap_or_else(|| BalanceSheet::default());
            atomic.checkpoint();
            let parcel = MessageContextParcel {
                atomic: atomic.derive(&IndexPointer::default()),
                runes: RuneTransfer::from_balance_sheet(initial_sheet.clone()),
                transaction: transaction.clone(),
                block: block.clone(),
                height,
                vout,
                pointer: self.pointer.unwrap_or_else(|| default_output),
                refund_pointer: self.pointer.unwrap_or_else(|| default_output),
                calldata: self
                    .message
                    .iter()
                    .map(|v| v.to_be_bytes())
                    .flatten()
                    .collect::<Vec<u8>>(),
                txindex,
                runtime_balances: Box::new(
                    balances_by_output
                        .get(&u32::MAX)
                        .map(|v| v.clone())
                        .unwrap_or_else(|| BalanceSheet::default()),
                ),
                sheets: Box::new(BalanceSheet::default()),
            };
            let pointer = self.pointer.unwrap_or_else(|| default_output);
            let refund_pointer = self.refund.unwrap_or_else(|| default_output);
            match T::handle(&parcel) {
                Ok(values) => match values.reconcile(balances_by_output, vout, pointer) {
                    Ok(_) => atomic.commit(),
                    Err(_) => {
                        let sheet = balances_by_output
                            .get(&vout)
                            .map(|v| v.clone())
                            .unwrap_or_else(|| BalanceSheet::default());
                        balances_by_output.remove(&vout);
                        if !balances_by_output.contains_key(&refund_pointer) {
                            balances_by_output.insert(refund_pointer, BalanceSheet::default());
                        }
                        sheet.pipe(balances_by_output.get_mut(&refund_pointer).unwrap());
                        atomic.rollback()
                    }
                },
                Err(_) => {
                    atomic.rollback();
                }
            }
        }
        Ok(())
    }
}

pub trait Protostones {
    fn burns(&self) -> Result<Vec<Protoburn>>;
    fn process_burns(
        &self,
        atomic: &mut AtomicPointer,
        runestone: &Runestone,
        runestone_output_index: u32,
        balances_by_output: &HashMap<u32, BalanceSheet>,
        proto_balances_by_output: &mut HashMap<u32, BalanceSheet>,
        default_output: u32,
        txid: Txid,
    ) -> Result<()>;
    fn encipher(&self) -> Result<Vec<u128>>;
}

impl Protostones for Vec<Protostone> {
    fn encipher(&self) -> Result<Vec<u128>> {
        let mut values = Vec::<u128>::new();
        for stone in self {
            values.push(stone.protocol_tag);
            let varints = stone.to_integers()?;
            values.push(varints.len() as u128);
            values.extend(&varints);
        }
        Ok(split_bytes(&encode_varint_list(&values)))
    }
    fn burns(&self) -> Result<Vec<Protoburn>> {
        Ok(self
            .into_iter()
            .filter(|stone| stone.burn.is_some())
            .map(|stone| Protoburn {
                tag: stone.burn.map(|v| v as u128),
                pointer: stone.pointer,
                from: stone.from.map(|v| vec![v]),
            })
            .collect())
    }
    fn process_burns(
        &self,
        atomic: &mut AtomicPointer,
        runestone: &Runestone,
        runestone_output_index: u32,
        balances_by_output: &HashMap<u32, BalanceSheet>,
        proto_balances_by_output: &mut HashMap<u32, BalanceSheet>,
        default_output: u32,
        txid: Txid,
    ) -> Result<()> {
        let mut burns = self.burns()?;
        burns.process(
            atomic,
            runestone.edicts.clone(),
            runestone_output_index,
            balances_by_output,
            proto_balances_by_output,
            default_output,
            txid,
        )?;
        Ok(())
    }
}
