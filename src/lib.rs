use std::fmt::Write;
use bitcoin::blockdata::block::Block;
use bitcoin::consensus::Decodable;
use metashrew_rs::{input, flush, stdio::{stdout}, println};
use protorune_rs::{message::{MessageContext}, Protorune};
use anyhow::{anyhow, Result};
use std::u128;

pub mod vm!
pub mod storage;

struct AlkaneMessageContext(());

// TODO: import MessageContextParcel

pub struct MessageContextParcel(u32);

impl MessageContext for AlkaneMessageContext {
  fn protocol_tag() -> u128 {
    1
  }
  /*
   * TODO: change protorune-rs to supply MessageContextParcel
  fn handle(data: &MessageContextParcel) -> bool {
    true
  }
  */
  fn handle() -> bool {
    true
  }
}

pub fn index_block() -> Result<()> {
  let data = input();
  let height = u32::from_le_bytes((&data[0..4]).try_into()?);
  let mut reader = &data[4..];
  let block = Block::consensus_decode(&mut reader)?;
  Protorune::index_block::<AlkaneMessageContext>(block, height)?;
  Ok(())
}

#[no_mangle]
pub fn _start() {
  index_block().unwrap();
  flush();
}
