use crate::utils::consume_sized_int;
use anyhow::{Result};

#[derive(Default, Clone)]
pub struct AlkaneId {
  block: u128,
  txindex: u128
}

impl AlkaneId {
  pub fn parse(cursor: &mut std::io::Cursor) ->  Result<AlkaneId> {
     let (block, txindex) = (consume_sized_int<u128>(cursor)?, consume_sized_int<u128>(cursor)?);
     Ok(AlkaneId {
       block,
       txindex
     })
  }
}
