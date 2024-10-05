use crate::utils::{consume_sized_int};
use anyhow::{Result};

#[derive(Default, Clone)]
pub struct AlkaneId {
  block: u128,
  tx: u128
}

impl AlkaneId {
  pub fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) ->  Result<AlkaneId> {
     let (block, tx) = (consume_sized_int::<u128>(cursor)?, consume_sized_int::<u128>(cursor)?);
     Ok(AlkaneId {
       block,
       tx
     })
  }
}

impl From<AlkaneId> for Vec<u8> {
    fn from(rune_id: AlkaneId) -> Self {
        let mut bytes = Vec::<u8>::with_capacity(32);
        bytes.extend(&rune_id.block.to_le_bytes());
        bytes.extend(&rune_id.tx.to_le_bytes());
        bytes
    }
}
