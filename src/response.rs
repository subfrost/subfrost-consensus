use crate::response::{AlkaneTransferParcel};

pub struct CallResponse {
  pub alkanes: AlkaneTransferParcel,
  pub data: Vec<u8>
}

impl CallResponse {
  pub fn parse(cursor: &mut std::io::Cursor) -> CallRespponse {
    let parcel = AlkaneTransferParcel::parse(cursor);
    CallResponse {
      alkanes: parcel,
      data: cursor.read_until_end()
    }
  }
}
