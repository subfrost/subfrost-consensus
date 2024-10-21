use anyhow::Result;
use metashrew_support::byte_view::ByteView;
use std::io::Read;
pub fn consume_sized_int<T: ByteView>(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<T> {
    let buffer = consume_exact(cursor, size_of::<T>())?;
    Ok(T::from_bytes(buffer))
}
pub fn consume_exact(cursor: &mut std::io::Cursor<Vec<u8>>, n: usize) -> Result<Vec<u8>> {
    let mut buffer: Vec<u8> = vec![0u8; n];
    cursor.read_exact(&mut buffer[0..n])?;
    Ok(buffer)
}
