use protorune::message::{MessageContext, MessageContextParcel};
use protorune::utils::{decode_varint_list};
use crate::vm;
use crate::cellpack::{Cellpack};
use std::io::{Cursor};
use anyhow::{Result};

#[derive(Clone, Default)]
pub struct AlkaneMessageContext(());

// TODO: import MessageContextParcel


const FUEL_LIMIT: u64 = 0x100000;

pub fn handle_message(myself: Box<MessageContextParcel>) -> Result<()> {
  let cellpack: Cellpack = decode_varint_list(&mut Cursor::new(myself.calldata.clone()))?.try_into()?;
  vm::run(vm::AlkanesRuntimeContext::from_parcel_and_cellpack(myself.as_ref(), &cellpack), &cellpack, FUEL_LIMIT)?;
  Ok(())
  
}
impl MessageContext for AlkaneMessageContext {
    fn protocol_tag() -> u128 {
        1
    }
    fn handle(_parcel: Box<MessageContextParcel>) -> bool {
      
      match handle_message(_parcel) {
        Ok(()) => true,
        Err(_) => false
      }
    }
}
