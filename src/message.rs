use crate::cellpack::Cellpack;
use crate::vm;
use anyhow::Result;
use protorune::{balance_sheet::{BalanceSheet}, rune_transfer::{RuneTransfer}, message::{MessageContext, MessageContextParcel}};
use protorune::utils::decode_varint_list;
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
    )?;
    Ok((vec![], BalanceSheet::default()))
}
impl MessageContext for AlkaneMessageContext {
    fn protocol_tag() -> u128 {
        1
    }
    fn handle(_parcel: &MessageContextParcel) -> Result<(Vec<RuneTransfer>, BalanceSheet)> {
        match handle_message(_parcel) {
            Ok((outgoing, runtime)) => { Ok((outgoing, runtime)) },
            Err(e) => Err(e),
        }
    }
}
