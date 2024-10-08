use metashrew::utils::consume_sized_int;
use protorune::balance_sheet::{ProtoruneRuneId}; 
use anyhow::Result;

#[derive(Default, Clone, Copy)]
pub struct AlkaneId {
    pub block: u128,
    pub tx: u128,
}

impl From<ProtoruneRuneId> for AlkaneId {
  fn from(id: ProtoruneRuneId) -> AlkaneId {
    AlkaneId {
      block: id.block,
      tx: id.tx
    }
  }
}



impl AlkaneId {
    pub fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<AlkaneId> {
        let (block, tx) = (
            consume_sized_int::<u128>(cursor)?,
            consume_sized_int::<u128>(cursor)?,
        );
        Ok(AlkaneId { block, tx })
    }
    pub fn new(block: u128, tx: u128) -> AlkaneId {
        AlkaneId { block, tx }
    }
    pub fn is_create(&self) -> bool {
        self.block == 1 && self.tx == 0
    }
    pub fn reserved(&self) -> Option<u128> {
        if self.block == 3 {
            Some(self.tx)
        } else {
            None
        }
    }
    pub fn factory(&self) -> Option<[u128; 2]> {
        if self.block == 5 {
            Some([1, self.tx])
        } else if self.block == 6 {
            Some([2, self.tx])
        } else {
            None
        }
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

impl Into<AlkaneId> for [u128; 2] {
    fn into(self) -> AlkaneId {
        AlkaneId {
            block: self[0],
            tx: self[1],
        }
    }
}

impl From<&AlkaneId> for Vec<u8> {
    fn from(rune_id: &AlkaneId) -> Self {
        let mut bytes = Vec::<u8>::with_capacity(32);
        bytes.extend(&rune_id.block.to_le_bytes());
        bytes.extend(&rune_id.tx.to_le_bytes());
        bytes
    }
}
