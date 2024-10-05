use std::io::{Cursor};
use crate::utils::{consume_sized_int};
use crate::id::{AlkaneId};
use anyhow::{Result};

#[derive(Clone, Default)]
struct Cellpack {
  target: AlkaneId;
  inputs: Vec<u128>;
}

impl Cellpack {
  pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<Cellpack> {
    let target = AlkaneId::parse(cursor)?
    let result = Cellpack::default();
    result.target = target;
    loop {
      match consume_sized_int::<u128>(cursor) {
        Ok(v) => result.inputs.push(v),
        Err(_) => { break; }
      }
    }
    Ok(result)
  }
}
