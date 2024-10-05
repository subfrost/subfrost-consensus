use anyhow::{Result};
use crate::parcel::{AlkaneTransferParcel};
use std::io::Read;

pub struct CallResponse {
  pub alkanes: AlkaneTransferParcel,
  pub data: Vec<u8>
}

impl CallResponse {
  pub fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<CallResponse> {
    let parcel = AlkaneTransferParcel::parse(cursor)?;
    Ok(CallResponse {
      alkanes: parcel,
      data: cursor.read(cursor.as_ref().len() as u64 - cursor.position())
    })
  }
}
