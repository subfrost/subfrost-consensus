use crate::utils::credit_balances;
use crate::vm;
use alkanes_support::cellpack::Cellpack;
use anyhow::Result;
use protorune::message::{MessageContext, MessageContextParcel};
use metashrew::index_pointer::{IndexPointer};
use metashrew::{stdio::stdout, println};
use protorune_support::{
    balance_sheet::BalanceSheet, rune_transfer::RuneTransfer, utils::decode_varint_list,
};
use std::io::Cursor;
use std::fmt::{Write};

#[derive(Clone, Default)]
pub struct AlkaneMessageContext(());

// TODO: import MessageContextParcel

const FUEL_LIMIT: u64 = 0x100000;

pub fn handle_message(parcel: &MessageContextParcel) -> Result<(Vec<RuneTransfer>, BalanceSheet)> {
    let cellpack: Cellpack =
        decode_varint_list(&mut Cursor::new(parcel.calldata.clone()))?.try_into()?;
    let mut context = vm::AlkanesRuntimeContext::from_parcel_and_cellpack(parcel, &cellpack);
    let mut atomic = parcel.atomic.derive(&IndexPointer::default());
    let (caller, myself) = vm::run_special_cellpacks(&mut context, &cellpack)?;
    credit_balances(&mut atomic, &myself.clone().into(), &parcel.runes);
    vm::prepare_context(&mut context, &caller, &myself, false);
    println!("prepare to execute\n");
    let response = vm::AlkanesInstance::from_alkane(context, FUEL_LIMIT)?.execute()?;
    println!("executed: {:?}\n", response);
    let mut combined = parcel.runtime_balances.as_ref().clone();
    <BalanceSheet as From<Vec<RuneTransfer>>>::from(parcel.runes.clone()).pipe(&mut combined);
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
            Err(e) => {
              println!("error: {}", e);
              Err(e)
            }
        }
    }
}
