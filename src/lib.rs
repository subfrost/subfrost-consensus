use crate::message::AlkaneMessageContext;
use anyhow::Result;
use bitcoin::blockdata::block::Block;
use bitcoin::consensus::Decodable;
use metashrew::{flush, input};
use protorune::Protorune;

pub mod cellpack;
pub mod envelope;
pub mod id;
pub mod message;
pub mod parcel;
pub mod response;
pub mod storage;
#[cfg(test)]
pub mod tests;
pub mod utils;
pub mod vm;

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
