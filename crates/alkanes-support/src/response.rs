use crate::parcel::AlkaneTransferParcel;
use anyhow::Result;
use metashrew_support::utils::consume_to_end;

#[derive(Default, Clone, Debug)]
pub struct CallResponse {
    pub alkanes: AlkaneTransferParcel,
    pub data: Vec<u8>,
}

impl CallResponse {
    pub fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<CallResponse> {
        let parcel = AlkaneTransferParcel::parse(cursor)?;
        Ok(CallResponse {
            alkanes: parcel,
            data: consume_to_end(cursor)?,
        })
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut list = Vec::<Vec<u128>>::new();
        list.push(vec![self.alkanes.0.len() as u128]);
        self.alkanes
            .0
            .iter()
            .for_each(|v| list.push(vec![v.id.block, v.id.tx, v.value]));
        let mut result: Vec<u8> = list
            .into_iter()
            .flatten()
            .map(|v| (&v.to_le_bytes()).to_vec())
            .flatten()
            .collect();
        result.extend(&self.data);
        result
    }
}
