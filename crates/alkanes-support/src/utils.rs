use crate::id::AlkaneId;
use anyhow::{anyhow, Result};

pub fn shift<T>(v: &mut Vec<T>) -> Option<T> {
    if v.is_empty() {
        None
    } else {
        Some(v.remove(0))
    }
}

pub fn shift_or_err(v: &mut Vec<u128>) -> Result<u128> {
    shift(v)
        .ok_or("")
        .map_err(|_| anyhow!("expected u128 value in list but list is exhausted"))
}

pub fn shift_id(v: &mut Vec<u128>) -> Result<AlkaneId> {
    let block = shift_or_err(v)?;
    let tx = shift_or_err(v)?;
    Ok(AlkaneId { block, tx })
}

pub fn shift_as_long(v: &mut Vec<u128>) -> Result<u64> {
    Ok(shift_or_err(v)?.try_into()?)
}

pub fn shift_bytes32(v: &mut Vec<u128>) -> Result<Vec<u8>> {
    Ok((&[
        shift_as_long(v)?,
        shift_as_long(v)?,
        shift_as_long(v)?,
        shift_as_long(v)?,
    ])
        .to_vec()
        .into_iter()
        .rev()
        .fold(Vec::<u8>::new(), |mut r, v| {
            r.extend(&v.to_be_bytes());
            r
        }))
}
