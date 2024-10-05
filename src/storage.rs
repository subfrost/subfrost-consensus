use std::collections::{HashMap};
use std::io::{Cursor};
use anyhow::{Result, anyhow};
use crate::utils::{consume_sized_int};

pub struct StorageMap(pub HashMap<Vec<u8>, Vec<u8>>)

impl FromIterator<(Vec<u8>, Vec<u8>)> for StorageMap {
  fn from_iter<I: IntoIterator<Item = (Vec<u8>, Vec<u8>)>>(iter: I) -> Self {
    Self(HashMap::<Vec<u8>, Vec<u8>>::from_iter(iter))
  }
}

impl StorageMap {
  pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<StorageMap> {
    let pairs = Vec::<(Vec<u8>, Vec<u8>)>::new();
    let len = consume_sized_int::<u32>(cursor)? as u64;
    let stop = cursor.position() + len;
    while cursor.position() < stop {
      let key: Vec<u8> = cursor.read(consume_sized_int::<u32>(cursor)?);
      let value: Vec<u8> = cursor.read(consume_sized_int::<u32>(cursor)?);
      pairs.push((key, value));
    }
    Ok(StorageMap::from_iter(pairs.into_iter()))
  }
  pub get(&self, k: AsRef<[u8]>) -> Vec<u8> {
    self.get(k.as_ref())
  }
  pub set(&self, k: AsRef<[u8]>, v: AsRef<[u8]>) {
    self.0.insert(k.as_ref().to_vec(), v.as_ref().to_vec());
  }
}
