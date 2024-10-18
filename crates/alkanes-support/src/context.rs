#[derive(Default, Clone)]
use alkanes_support::id::{AlkaneId};
use alkanes_support::parcel::{AlkaneTransferParcel};
use metashrew_support::utils::{consume_sized_int};
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct Context {
    pub myself: AlkaneId,
    pub caller: AlkaneId,
    pub incoming_alkanes: AlkaneTransferParcel,
    pub inputs: Vec<u128>
}

impl Context {
  pub fn parse(v: &mut Cursor<Vec<u8>>) -> Result<Context> {
    let mut result = Context::default();
    result.myself = AlkaneId::parse(v)?;
    result.caller = AlkaneId::parse(v)?;
    result.incoming_alkanes = AlkaneTransferParcel::parse(v)?;
    while !v.is_empty() {
      result.inputs.push(consume_sized_int::<u128>(v));
    }
    result
  }
}
