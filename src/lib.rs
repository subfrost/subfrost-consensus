use crate::message::AlkaneMessageContext;
use anyhow::Result;
use bitcoin::blockdata::block::Block;
use bitcoin::consensus::Decodable;
use metashrew::{flush, input};
use protorune::{message::{MessageContext}, Protorune};
use protorune_support::protostone::{Protostone};
use ordinals::{Runestone, Artifact};

pub mod message;
#[cfg(test)]
pub mod tests;
pub mod utils;
pub mod vm;

use crate::vm::fuel::{set_message_count};

pub fn count_alkanes_protomessages(block: &Block) {
  let mut count: u64 = 0;
  for tx in &block.txdata {
    if let Some(Artifact::Runestone(ref runestone)) = Runestone::decipher(tx) {
      if let Ok(protostones) = Protostone::from_runestone(runestone) {
        for protostone in protostones {
          if protostone.protocol_tag == AlkaneMessageContext::protocol_tag() && protostone.message.len() != 0 {
            count = count + 1;
          }
        }
      }
    }
  }
  set_message_count(count);
}

pub fn index_block(block: &Block, height: u32) -> Result<()> {
    count_alkanes_protomessages(&block);
    Protorune::index_block::<AlkaneMessageContext>(block.clone(), height.into())?;
    Ok(())
}

#[no_mangle]
pub fn _start() {
    let data = input();
    let height = u32::from_le_bytes((&data[0..4]).try_into().unwrap());
    let mut reader = &data[4..];
    let block = Block::consensus_decode(&mut reader).unwrap();
    index_block(&block, height).unwrap();
    flush();
}
