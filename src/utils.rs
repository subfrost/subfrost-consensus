use anyhow::{Result};
use metashrew::byte_view::{ByteView};
use std::mem::{size_of};
use std::io::BufRead;
use std::io::Read;

pub fn consume_sized_int<T: ByteView>(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<T> {
  let buffer = consume_exact(cursor, size_of::<T>())?;
  Ok(T::from_bytes(buffer))
}

pub fn consume_to_end(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<Vec<u8>> {
  let mut result = vec![0u8; (cursor.get_ref().len() as u64 - cursor.position()).try_into()?];
  cursor.read_to_end(&mut result)?;
  cursor.consume((cursor.get_ref().len() as u64 - cursor.position()).try_into()?);
  Ok(result)
}

pub fn consume_exact(cursor: &mut std::io::Cursor<Vec<u8>>, n: usize) -> Result<Vec<u8>> {
  let mut buffer: Vec<u8> = vec![0u8; n];
  cursor.read_exact(&mut buffer[0..n])?;
  cursor.consume(n);
  Ok(buffer)
}

pub fn consume_u128(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<u128> {
  consume_sized_int::<u128>(cursor)
}
