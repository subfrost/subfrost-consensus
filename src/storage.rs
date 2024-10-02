use std::collections::{HashMap};
use std::io::{Cursor};
use anyhow::{Result, anyhow};

pub struct StorageMap(pub HashMap<Vec<u8>, Vec<u8>>)

impl StorageMap {
  pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<StorageMap> {
    let pairs = Vec::<(Vec<u8>, Vec<u8>)>::new();
    let len = consume_sized_int::<u32>(cursor)? as u64;
    let stop = cursor.position() + len;
    while cursor.position() < stop {
      let key = cursor.read(consume_sized_int::<u32>(cursor)?);
      let value = cursor.read(consume_sized_int::<u32>(cursor)?);
    }

  }
