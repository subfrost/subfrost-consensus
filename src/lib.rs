use alkanes::view::simulate_parcel;
use subfrost_support::proto;
use anyhow::Result;
use bitcoin::blockdata::block::Block;
use bitcoin::hashes::Hash;
#[allow(unused_imports)]
use metashrew::{
    flush, input, println,
    stdio::{stdout, Write},
};
use metashrew_support::block::AuxpowBlock;
use metashrew_support::compat::{to_arraybuffer_layout, to_passback_ptr};
use protobuf::{Message, MessageField};
use protorune::message::MessageContextParcel;
use std::io::Cursor;
#[cfg(test)]
pub mod tests;

pub fn index_block(block: &Block, height: u32) -> Result<()> {
    alkanes::indexer::index_block(block, height)?;
    Ok(())
}

#[no_mangle]
pub fn receipts() -> i32 {
    let data = input();
    let _height = u32::from_le_bytes((&data[0..4]).try_into().unwrap());
    let reader = &data[4..];
    let mut result: proto::subfrost::ReceiptsResponse = proto::subfrost::ReceiptsResponse::new();
    to_passback_ptr(&mut to_arraybuffer_layout::<&[u8]>(
        result.write_to_bytes().unwrap().as_ref(),
    ))
}

#[no_mangle]
pub fn _start() {
    let data = input();
    let height = u32::from_le_bytes((&data[0..4]).try_into().unwrap());
    let reader = &data[4..];
    let block: Block = AuxpowBlock::parse(&mut Cursor::<Vec<u8>>::new(reader.to_vec()))
        .unwrap()
        .to_consensus();
    index_block(&block, height).unwrap();
    flush();
}
