use crate::id::{AlkaneId};
use anyhow::Result;
use metashrew_support::utils::{consume_sized_int};
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
}

impl TryFrom<Vec<u128>> for Cellpack {
    type Error = anyhow::Error;
    fn try_from(v: Vec<u128>) -> std::result::Result<Cellpack, Self::Error> {
        Ok(Cellpack {
            target: <[u128; 2] as TryFrom<&[u128]>>::try_from(&v[0..2])?.into(),
            inputs: (&v[2..]).to_vec(),
        })
    }
}
