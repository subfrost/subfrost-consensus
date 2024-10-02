use crate::response::{AlkaneTransferParcel};

pub struct CallResponse {
  pub alkanes: AlkaneTransferParcel,
  pub data: Vec<u8>
}

impl CallResponse {
  pub fn parse(cursor: &mut std::io::Cursor<Vec<u8>) -> Result<CallRespponse> {
    let parcel = AlkaneTransferParcel::parse(cursor)?;
    CallResponse {
      alkanes: parcel,
      data: cursor.read(cursor.as_ref().len() as u64 - cursor.position())
    }
  }
}
