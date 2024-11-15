use crate::utils::{credit_balances, debit_balances, pipe_storagemap_to};
use crate::vm::{
    fuel::start_fuel,
    runtime::AlkanesRuntimeContext,
    utils::{prepare_context, run_after_special, run_special_cellpacks},
};
use alkanes_support::cellpack::Cellpack;
use anyhow::Result;
use metashrew::index_pointer::IndexPointer;
use metashrew::{
    println,
    stdio::{stdout, Write},
};
use metashrew_support::index_pointer::KeyValuePointer;
use protorune::balance_sheet::CheckedDebit;
use protorune::message::{MessageContext, MessageContextParcel};
use protorune_support::{
    balance_sheet::BalanceSheet, rune_transfer::RuneTransfer, utils::decode_varint_list,
};
use std::io::Cursor;

#[derive(Clone, Default)]
pub struct AlkaneMessageContext(());

// TODO: import MessageContextParcel

pub fn handle_message(parcel: &MessageContextParcel) -> Result<(Vec<RuneTransfer>, BalanceSheet)> {
    let cellpack: Cellpack =
        decode_varint_list(&mut Cursor::new(parcel.calldata.clone()))?.try_into()?;
    let mut context = AlkanesRuntimeContext::from_parcel_and_cellpack(parcel, &cellpack);
    let mut atomic = parcel.atomic.derive(&IndexPointer::default());
    println!("incoming runes: {:?}", parcel.runes);
    let (caller, myself, binary) = run_special_cellpacks(&mut context, &cellpack)?;
    credit_balances(&mut atomic, &myself, &parcel.runes);
    prepare_context(&mut context, &caller, &myself, false);
    let (response, _gas_used) = run_after_special(context, binary, start_fuel())?;
    pipe_storagemap_to(
        &response.storage,
        &mut atomic.derive(&IndexPointer::from_keyword("/alkanes/").select(&myself.clone().into())),
    );
    let mut combined = parcel.runtime_balances.as_ref().clone();
    <BalanceSheet as From<Vec<RuneTransfer>>>::from(parcel.runes.clone()).pipe(&mut combined);
    let sheet = <BalanceSheet as From<Vec<RuneTransfer>>>::from(response.alkanes.clone().into());
    combined.debit_checked(&sheet, &mut atomic)?;
    debit_balances(&mut atomic, &myself, &response.alkanes)?;
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
                panic!("{:?}", e); // Print the error
            }
        }
    }
}
