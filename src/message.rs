use crate::vm;
use alkanes_support::cellpack::Cellpack;
use anyhow::Result;
use protorune::message::{MessageContext, MessageContextParcel};
use protorune_support::{
    balance_sheet::BalanceSheet, rune_transfer::RuneTransfer, utils::decode_varint_list,
};
use std::io::Cursor;

#[derive(Clone, Default)]
pub struct AlkaneMessageContext(());

// TODO: import MessageContextParcel

const FUEL_LIMIT: u64 = 0x100000;

pub fn handle_message(myself: &MessageContextParcel) -> Result<(Vec<RuneTransfer>, BalanceSheet)> {
    let cellpack: Cellpack =
        decode_varint_list(&mut Cursor::new(myself.calldata.clone()))?.try_into()?;
    let response = vm::run(
        vm::AlkanesRuntimeContext::from_parcel_and_cellpack(myself, &cellpack),
        &cellpack,
        FUEL_LIMIT,
        false
    )?;
    let mut combined = myself.runtime_balances.as_ref().clone();
    <BalanceSheet as From<Vec<RuneTransfer>>>::from(myself.runes.clone()).pipe(&mut combined);
    let sheet = <BalanceSheet as From<Vec<RuneTransfer>>>::from(response.alkanes.clone().into());
    combined.debit(&sheet)?;
    Ok((response.alkanes.into(), combined))
}
impl MessageContext for AlkaneMessageContext {
    fn protocol_tag() -> u128 {
        1
    }
    fn handle(_parcel: &MessageContextParcel) -> Result<(Vec<RuneTransfer>, BalanceSheet)> {
        match handle_message(_parcel) {
            Ok((outgoing, runtime)) => Ok((outgoing, runtime)),
            Err(e) => Err(e),
        }
    }
}
