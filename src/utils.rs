use anyhow::Result;
use metashrew::byte_view::ByteView;
use std::io::BufRead;
use std::io::Read;
use std::mem::size_of;
use ordinals::varint;

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

pub fn is_empty(cursor: &mut std::io::Cursor<Vec<u8>>) -> bool {
  cursor.position() >= cursor.get_ref().len() as u64
}

pub fn remaining_slice(cursor: &mut std::io::Cursor<Vec<u8>>) -> &[u8] {
  &cursor.get_ref()[(cursor.position() as usize)..cursor.get_ref().len()]
}

pub fn decode_varint_list(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<Vec<u128>> {
  let mut result: Vec<u128> = vec![];
  while !is_empty(cursor) {
    let (n, sz) = varint::decode(remaining_slice(cursor))?;
    cursor.consume(sz);
    result.push(n);
  }
  Ok(result)
}
