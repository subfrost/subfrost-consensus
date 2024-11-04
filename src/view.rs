use crate::utils::{credit_balances, debit_balances, pipe_storagemap_to};
use crate::vm::runtime::AlkanesRuntimeContext;
use crate::vm::utils::{prepare_context, run_after_special, run_special_cellpacks};
use alkanes_support::cellpack::Cellpack;
use alkanes_support::response::ExtendedCallResponse;
use anyhow::Result;
use metashrew::index_pointer::IndexPointer;
use metashrew_support::index_pointer::KeyValuePointer;
use protorune::message::MessageContextParcel;
use protorune_support::balance_sheet::BalanceSheet;
use protorune_support::rune_transfer::RuneTransfer;
use protorune_support::utils::decode_varint_list;
use std::io::Cursor;

pub fn simulate_parcel(parcel: &MessageContextParcel) -> Result<(ExtendedCallResponse, u64)> {
    let cellpack: Cellpack =
        decode_varint_list(&mut Cursor::new(parcel.calldata.clone()))?.try_into()?;
    let mut context = AlkanesRuntimeContext::from_parcel_and_cellpack(parcel, &cellpack);
    let mut atomic = parcel.atomic.derive(&IndexPointer::default());
    let (caller, myself, binary) = run_special_cellpacks(&mut context, &cellpack)?;
    credit_balances(&mut atomic, &myself, &parcel.runes);
    prepare_context(&mut context, &caller, &myself, false);
    let (response, gas_used) = run_after_special(context, binary, u64::MAX)?;
    pipe_storagemap_to(
        &response.storage,
        &mut atomic.derive(&IndexPointer::from_keyword("/alkanes/").select(&myself.clone().into())),
    );
    let mut combined = parcel.runtime_balances.as_ref().clone();
    <BalanceSheet as From<Vec<RuneTransfer>>>::from(parcel.runes.clone()).pipe(&mut combined);
    let sheet = <BalanceSheet as From<Vec<RuneTransfer>>>::from(response.alkanes.clone().into());
    combined.debit(&sheet)?;
    debit_balances(&mut atomic, &myself, &response.alkanes)?;
    Ok((response, gas_used))
}
