use anyhow::{Result};
use crate::parcel::{AlkaneTransferParcel};
use crate::utils::{consume_to_end};

#[derive(Default, Clone)]
pub struct CallResponse {
  pub alkanes: AlkaneTransferParcel,
  pub data: Vec<u8>
}

impl CallResponse {
  pub fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<CallResponse> {
    let parcel = AlkaneTransferParcel::parse(cursor)?;
    Ok(CallResponse {
      alkanes: parcel,
      data: consume_to_end(cursor)?
    })
  }
}
