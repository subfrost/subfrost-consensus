use bitcoin::blockdata::block::Block;
use bitcoin::consensus::Decodable;
use metashrew::{input, flush, stdio::{stdout}, println};
use protorune::{message::{MessageContextParcel, MessageContext}, Protorune};
use anyhow::{Result};
use std::u128;
use crate::{id::{AlkaneId}, message::{AlkaneMessageContext}};


pub mod vm;
pub mod storage;
pub mod utils;
pub mod response;
pub mod parcel;
pub mod cellpack;
pub mod id;
pub mod message;
pub mod envelope;
pub fn index_block() -> Result<()> {
  let data = input();
  let height = u32::from_le_bytes((&data[0..4]).try_into()?);
  let mut reader = &data[4..];
  let block = Block::consensus_decode(&mut reader)?;
  Protorune::index_block::<AlkaneMessageContext>(block, height.into())?;
  Ok(())
}

#[no_mangle]
pub fn _start() {
  index_block().unwrap();
  flush();
}
