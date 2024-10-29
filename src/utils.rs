use alkanes_support::id::AlkaneId;
use alkanes_support::parcel::AlkaneTransferParcel;
use alkanes_support::storage::StorageMap;
use anyhow::{anyhow, Result};
use protorune_support::{rune_transfer::{RuneTransfer}};
use metashrew::index_pointer::{AtomicPointer, IndexPointer};
use metashrew::{println, stdio::{stdout}};
use metashrew_support::index_pointer::KeyValuePointer;
use std::sync::Arc;
use std::fmt::{Write};

pub fn balance_pointer(
    atomic: &mut AtomicPointer,
    who: &AlkaneId,
    what: &AlkaneId,
) -> AtomicPointer {
    atomic
        .derive(&IndexPointer::default())
        .keyword("/alkanes/")
        .select(&what.clone().into())
        .keyword("/balances/")
        .select(&who.clone().into())
}

pub fn credit_balances(atomic: &mut AtomicPointer, to: &AlkaneId, runes: &Vec<RuneTransfer>) {
    for rune in runes.clone() {
        balance_pointer(atomic, to, &rune.id.clone().into()).set_value::<u128>(rune.value);
    }
}

pub fn transfer_from(
    parcel: &AlkaneTransferParcel,
    atomic: &mut AtomicPointer,
    from: &AlkaneId,
    to: &AlkaneId,
) -> Result<()> {
    println!("parcel: {:?}", parcel);
    for transfer in &parcel.0 {
        let mut from_pointer = balance_pointer(atomic, &from.clone().into(), &transfer.id.clone().into());
        let balance = from_pointer.get_value::<u128>();
        println!("balance: {}\n", balance);
        if balance < transfer.value {
            return Err(anyhow!("balance underflow"));
        }
        from_pointer.set_value::<u128>(balance - transfer.value);
        balance_pointer(atomic, &to.clone().into(), &transfer.id.clone().into()).set_value::<u128>(balance + transfer.value);
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
