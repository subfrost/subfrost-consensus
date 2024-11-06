use crate::message::AlkaneMessageContext;
use crate::utils::u128_from_bytes;
use crate::view::simulate_parcel;
use alkanes_support::id::AlkaneId;
use alkanes_support::response::ExtendedCallResponse;
use anyhow::Result;
use bitcoin::blockdata::block::Block;
use bitcoin::blockdata::transaction::Transaction;
use bitcoin::consensus::Decodable;
use metashrew::{flush, input};
use metashrew_support::block::AuxpowBlock;
use metashrew_support::compat::{to_arraybuffer_layout, to_passback_ptr};
use ordinals::{Artifact, Runestone};
use protobuf::{Message, MessageField, SpecialFields};
use protorune::message::MessageContextParcel;
use protorune::{message::MessageContext, Protorune};
use protorune_support::balance_sheet::ProtoruneRuneId;
use protorune_support::protostone::Protostone;
use protorune_support::rune_transfer::RuneTransfer;
use protorune_support::utils::consensus_decode;
use std::io::Cursor;
pub mod message;
pub mod proto;
#[cfg(test)]
pub mod tests;
pub mod utils;
pub mod view;
pub mod vm;

use crate::vm::fuel::set_message_count;

pub fn count_alkanes_protomessages(block: &Block) {
    let mut count: u64 = 0;
    for tx in &block.txdata {
        if let Some(Artifact::Runestone(ref runestone)) = Runestone::decipher(tx) {
            if let Ok(protostones) = Protostone::from_runestone(runestone) {
                for protostone in protostones {
                    if protostone.protocol_tag == AlkaneMessageContext::protocol_tag()
                        && protostone.message.len() != 0
                    {
                        count = count + 1;
                    }
                }
            }
        }
    }
    set_message_count(count);
}

pub fn index_block(block: &Block, height: u32) -> Result<()> {
    count_alkanes_protomessages(&block);
    Protorune::index_block::<AlkaneMessageContext>(block.clone(), height.into())?;
    Ok(())
}

impl Into<MessageContextParcel> for proto::alkanes::MessageContextParcel {
    fn into(self) -> MessageContextParcel {
        let mut result = MessageContextParcel::default();
        result.height = self.height;
        result.block = consensus_decode::<Block>(&mut Cursor::new(self.block)).unwrap();
        result.transaction =
            consensus_decode::<Transaction>(&mut Cursor::new(self.transaction)).unwrap();
        result.vout = self.vout;
        result.calldata = self.calldata;
        result.runes = self
            .alkanes
            .into_iter()
            .map(|v| RuneTransfer {
                id: ProtoruneRuneId {
                    block: u128_from_bytes(v.id.block.clone()),
                    tx: u128_from_bytes(v.id.tx.clone()),
                },
                value: u128_from_bytes(v.value.clone()),
            })
            .collect::<Vec<RuneTransfer>>();
        result.pointer = self.pointer;
        result.refund_pointer = self.refund_pointer;
        result
    }
}

impl Into<proto::alkanes::ExtendedCallResponse> for ExtendedCallResponse {
    fn into(self) -> proto::alkanes::ExtendedCallResponse {
        let mut result: proto::alkanes::ExtendedCallResponse =
            proto::alkanes::ExtendedCallResponse::new();
        result.storage = self
            .storage
            .0
            .into_iter()
            .map(|(key, value)| proto::alkanes::KeyValuePair {
                key,
                value,
                special_fields: SpecialFields::new(),
            })
            .collect::<Vec<proto::alkanes::KeyValuePair>>();
        result.data = self.data;
        result.alkanes = self
            .alkanes
            .0
            .into_iter()
            .map(|v| proto::alkanes::AlkaneTransfer {
                id: MessageField::some(proto::alkanes::AlkaneId {
                    block: v.id.block.to_le_bytes().to_vec(),
                    tx: v.id.tx.to_le_bytes().to_vec(),
                    special_fields: SpecialFields::new(),
                }),
                special_fields: SpecialFields::new(),
                value: v.value.to_le_bytes().to_vec(),
            })
            .collect::<Vec<proto::alkanes::AlkaneTransfer>>();

        result
    }
}

impl Into<proto::alkanes::AlkaneId> for AlkaneId {
    fn into(self) -> proto::alkanes::AlkaneId {
        proto::alkanes::AlkaneId {
            block: self.block.to_le_bytes().to_vec(),
            tx: self.tx.to_le_bytes().to_vec(),
            special_fields: SpecialFields::new(),
        }
    }
}

impl Into<proto::alkanes::AlkaneInventoryRequest> for AlkaneId {
    fn into(self) -> proto::alkanes::AlkaneInventoryRequest {
        proto::alkanes::AlkaneInventoryRequest {
            id: MessageField::some(proto::alkanes::AlkaneId {
                block: self.block.to_le_bytes().to_vec(),
                tx: self.tx.to_le_bytes().to_vec(),
                special_fields: SpecialFields::new(),
            }),
            special_fields: SpecialFields::new(),
        }
    }
}

#[no_mangle]
pub fn simulate() -> i32 {
    let data = input();
    let _height = u32::from_le_bytes((&data[0..4]).try_into().unwrap());
    let reader = &data[4..];
    let mut result: proto::alkanes::SimulateResponse = proto::alkanes::SimulateResponse::new();
    let (response, gas_used) = simulate_parcel(
        &proto::alkanes::MessageContextParcel::parse_from_bytes(reader)
            .unwrap()
            .into(),
    )
    .unwrap();
    result.execution = MessageField::some(response.into());
    result.gas_used = gas_used;
    to_passback_ptr(&mut to_arraybuffer_layout::<&[u8]>(
        result.write_to_bytes().unwrap().as_ref(),
    ))
}

// #[no_mangle]
// pub fn alkane_balance_sheet() -> i32 {
//     let data = input();
//     let _height = u32::from_le_bytes((&data[0..4]).try_into().unwrap());
//     let reader = &data[4..];
//     let mut result: proto::alkanes::SimulateResponse = proto::alkanes::SimulateResponse::new();
//     let (response, gas_used) = alkane_inventory(
//         &proto::alkanes::MessageContextParcel::parse_from_bytes(reader).unwrap().into()
//     ).unwrap();
//     result.execution = MessageField::some(response.into());
//     result.gas_used = gas_used;
//     to_passback_ptr(&mut to_arraybuffer_layout::<&[u8]>(result.write_to_bytes().unwrap().as_ref()))
// }

#[no_mangle]
pub fn _start() {
    let data = input();
    let height = u32::from_le_bytes((&data[0..4]).try_into().unwrap());
    let mut reader = &data[4..];
    let block: Block = AuxpowBlock::parse(&mut Cursor::<Vec<u8>>::new(reader.to_vec()))
        .unwrap()
        .to_consensus();
    index_block(&block, height).unwrap();
    flush();
}
