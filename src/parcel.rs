use crate::id::AlkaneId;
use crate::utils::consume_sized_int;
use anyhow::Result;

#[derive(Default, Clone)]
pub struct AlkaneTransfer {
    pub id: AlkaneId,
    pub value: u128,
}

#[derive(Default, Clone)]
pub struct AlkaneTransferParcel(pub Vec<AlkaneTransfer>);

impl AlkaneTransferParcel {
    pub fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<AlkaneTransferParcel> {
        let mut result = AlkaneTransferParcel::default();
        for _i in [0..consume_sized_int::<u128>(cursor)?] {
            result.0.push(AlkaneTransfer::parse(cursor)?);
        }
        Ok(result)
    }
}

impl AlkaneTransfer {
    pub fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<AlkaneTransfer> {
        let id = AlkaneId::parse(cursor)?;
        let value = consume_sized_int::<u128>(cursor)?;
        Ok(AlkaneTransfer { id, value })
    }
}
