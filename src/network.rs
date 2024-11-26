#[allow(unused_imports)]
use crate::precompiled::alkanes_std_genesis_alkane_regtest_build;
#[allow(unused_imports)]
use crate::precompiled::alkanes_std_genesis_alkane_build;
use crate::utils::pipe_storagemap_to;
use crate::{simulate_parcel, AlkaneMessageContext};
use alkanes_support::cellpack::Cellpack;
use alkanes_support::id::AlkaneId;
use alkanes_support::parcel::AlkaneTransferParcel;
use anyhow::Result;
use bitcoin::hashes::Hash;
use bitcoin::{Block, OutPoint, Transaction, Txid};
use metashrew::index_pointer::{AtomicPointer, IndexPointer};
use metashrew_support::index_pointer::KeyValuePointer;
use protorune::balance_sheet::PersistentRecord;
use protorune::message::{MessageContext, MessageContextParcel};
use protorune::tables::RuneTable;
use protorune_support::balance_sheet::BalanceSheet;
use protorune_support::utils::outpoint_encode;
use std::sync::Arc;

#[cfg(feature = "regtest")]
pub fn genesis_alkane_bytes() -> Vec<u8> {
  alkanes_std_genesis_alkane_regtest_build::get_bytes()
}

#[cfg(not(feature = "regtest"))]
pub fn genesis_alkane_bytes() -> Vec<u8> {
  alkanes_std_genesis_alkane_build::get_bytes()
}

#[cfg(feature = "regtest")]
pub const GENESIS_BLOCK: u64 = 0;

#[cfg(not(feature = "regtest"))]
pub const GENESIS_BLOCK: u64 = 880_000;

pub fn is_active(height: u64) -> bool {
    height >= GENESIS_BLOCK
}

pub fn is_genesis(height: u64) -> bool {
    height == GENESIS_BLOCK
}

pub fn genesis(block: &Block) -> Result<()> {
    IndexPointer::from_keyword("/alkanes/")
        .select(&(AlkaneId { block: 2, tx: 0 }).into())
        .set(Arc::new(alkanes_std_genesis_alkane_build::get_bytes()));
    IndexPointer::from_keyword("/sequence").set_value::<u128>(1);
    let mut atomic: AtomicPointer = AtomicPointer::default();
    let myself = AlkaneId { block: 2, tx: 0 };
    let parcel = MessageContextParcel {
        atomic: atomic.derive(&IndexPointer::default()),
        runes: vec![],
        transaction: Transaction {
            version: bitcoin::blockdata::transaction::Version::ONE,
            input: vec![],
            output: vec![],
            lock_time: bitcoin::absolute::LockTime::ZERO,
        },
        block: block.clone(),
        height: GENESIS_BLOCK,
        pointer: 0,
        refund_pointer: 0,
        calldata: (Cellpack {
            target: myself.clone(),
            inputs: vec![0],
        })
        .encipher(),
        sheets: Box::<BalanceSheet>::new(BalanceSheet::default()),
        txindex: 0,
        vout: 0,
        runtime_balances: Box::<BalanceSheet>::new(BalanceSheet::default()),
    };
    let (response, _gas_used) = simulate_parcel(&parcel)?;
    <AlkaneTransferParcel as Into<BalanceSheet>>::into(response.alkanes.into()).save(
        &mut atomic.derive(
            &RuneTable::for_protocol(AlkaneMessageContext::protocol_tag())
                .OUTPOINT_TO_RUNES
                .select(&outpoint_encode(&OutPoint {
                    txid: Txid::from_byte_array(
                        <Vec<u8> as AsRef<[u8]>>::as_ref(&hex::decode(
                            "3977b30a97c9b9d609afb4b7cc138e17b21d1e0c5e360d25debf1441de933bf4",
                        )?)
                        .try_into()?,
                    ),
                    vout: 0,
                })?),
        ),
        false,
    );
    pipe_storagemap_to(
        &response.storage,
        &mut atomic.derive(&IndexPointer::from_keyword("/alkanes/").select(&myself.clone().into())),
    );
    atomic.commit();
    Ok(())
}
