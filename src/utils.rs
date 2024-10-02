use anyhow::{Result};

pub fn consume_sized_int<T: ByteView>(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<T> {
  let remaining: u64 = cursor.as_ref().len() as u64 - cursor.position();
  let requested = std::mem::size_of<T>();
  if remaining < requested {
    Err(anyhow!(format!("{} bytes requested but only {} remain", requested, remaining)))
  } else {
    Ok(u128::from_le_bytes(cursor.read(16)))
  }
}
