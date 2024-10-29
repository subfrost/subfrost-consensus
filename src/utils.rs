use alkanes_support::id::AlkaneId;
use alkanes_support::parcel::AlkaneTransferParcel;
use alkanes_support::storage::StorageMap;
use anyhow::{anyhow, Result};
use protorune_support::{rune_transfer::{RuneTransfer}};
use metashrew::index_pointer::{AtomicPointer, IndexPointer};
use metashrew_support::index_pointer::KeyValuePointer;
use std::sync::Arc;

pub fn balance_pointer(
    atomic: &mut AtomicPointer,
    who: &AlkaneId,
    what: &AlkaneId,
) -> AtomicPointer {
    atomic
        .derive(&IndexPointer::default())
        .keyword("/alkanes/")
        .select(&who.clone().into())
        .keyword("/balances/")
        .select(&what.clone().into())
}

pub fn credit_balances(atomic: &mut AtomicPointer, to: &AlkaneId, runes: &Vec<RuneTransfer>) {
    for rune in runes.clone() {
        balance_pointer(atomic, to, &rune.id.clone().into()).set_value::<u128>(rune.value);
    }
}

pub fn transfer_from<T: KeyValuePointer>(
    parcel: &AlkaneTransferParcel,
    pointer: &mut T,
    from: &AlkaneId,
    to: &AlkaneId,
) -> Result<()> {
    for transfer in &parcel.0 {
        let balance = pointer
            .keyword("/alkanes/")
            .select(&transfer.id.into())
            .keyword("/balances/")
            .select(&from.into())
            .get_value::<u128>();
        if balance < transfer.value {
            return Err(anyhow!("balance underflow"));
        }
        pointer
            .keyword("/alkanes/")
            .select(&transfer.id.into())
            .keyword("/balances/")
            .select(&from.into())
            .set_value::<u128>(balance - transfer.value);
        pointer
            .keyword("/alkanes/")
            .select(&transfer.id.into())
            .keyword("/balances/")
            .select(&to.into())
            .set_value::<u128>(balance + transfer.value);
    }
    Ok(())
}
pub fn pipe_storagemap_to<T: KeyValuePointer>(map: &StorageMap, pointer: &mut T) {
    map.0.iter().for_each(|(k, v)| {
        pointer
            .keyword("/storage/")
            .select(k)
            .set(Arc::new(v.clone()));
    });
}
