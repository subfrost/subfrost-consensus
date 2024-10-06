use crate::id::AlkaneId;
use crate::utils::consume_sized_int;
use anyhow::Result;
use std::io::Cursor;

#[derive(Clone, Default)]
pub struct Cellpack {
    pub target: AlkaneId,
    pub inputs: Vec<u128>,
}

impl Cellpack {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<Cellpack> {
        let target = AlkaneId::parse(cursor)?;
        let mut result = Cellpack::default();
        result.target = target;
        loop {
            match consume_sized_int::<u128>(cursor) {
                Ok(v) => result.inputs.push(v),
                Err(_) => {
                    break;
                }
            }
        }
        Ok(result)
    }
    pub fn is_create(&self) -> bool {
        self.target.block == 0 && self.target.tx == 0
    }
}
