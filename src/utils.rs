use alkanes_support::id::AlkaneId;
use alkanes_support::parcel::AlkaneTransferParcel;
use alkanes_support::storage::StorageMap;
use anyhow::{anyhow, Result};
use metashrew::index_pointer::KeyValuePointer;
use std::sync::Arc;
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
