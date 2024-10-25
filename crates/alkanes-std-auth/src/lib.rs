use alkanes_runtime::{storage::{StoragePointer}, runtime::AlkaneResponder};
use metashrew_support::index_pointer::{KeyValuePointer};
use alkanes_support::{
    cellpack::Cellpack, context::Context, parcel::AlkaneTransfer, response::CallResponse,
    witness::find_witness_payload,
};
use alkanes_support::{
    println,
    stdio::{stdout}
};
use std::sync::{Arc};
use std::fmt::Write;
use anyhow::{anyhow, Result};
use bitcoin::blockdata::transaction::Transaction;
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};
use protorune_support::utils::consensus_decode;

pub trait Token {
  fn name(&self) -> String;
  fn symbol(&self) -> String;
}

fn shift<T>(v: &mut Vec<T>) -> Option<T> {
    if v.is_empty() {
        None
    } else {
        Some(v.remove(0))
    }
}

#[derive(Default)]
pub struct AuthToken(());

impl Token for AuthToken {
  fn name(&self) -> String { String::from("AUTH") }
  fn symbol(&self) -> String { String::from("AUTH") }
}

impl AlkaneResponder for AuthToken {
    fn execute(&self) -> CallResponse {
        let mut context = self.context().unwrap();
        let mut inputs = context.inputs.clone();
        match shift(&mut inputs).unwrap() {
            0 => {
                let mut pointer = StoragePointer::from_keyword("/initialized");
                if pointer.get().len() != 0 {
                    let amount = shift(&mut inputs).unwrap();
                    let mut response: CallResponse = CallResponse::default();
                    response.alkanes = context.incoming_alkanes.clone();
                    response.alkanes.0.push(AlkaneTransfer {
                        id: context.myself.clone(),
                        value: amount,
                    });
                    pointer.set(Arc::new(vec![0x01]));
                    response
                } else {
                    panic!("already initialized");
                }
            }
            1 => {
              let mut response = CallResponse::default();
              if context.incoming_alkanes.0.len() != 1 {
                panic!("did not authenticate with only the authentication token");
              }
              let transfer = context.incoming_alkanes.0[0].clone();
              if transfer.id != context.myself.clone() {
                panic!("supplied alkane is not authentication token");
              }
              if transfer.value < 1 {
                panic!("less than 1 unit of authentication token supplied to authenticate");
              }
              response.data = vec![0x01];
              response.alkanes.0.push(transfer);
              response
            }
            99 => {
              let mut response = CallResponse::default();
              response.data = self.name().into_bytes().to_vec();
              response
            }
            100 => {
              let mut response = CallResponse::default();
              response.data = self.symbol().into_bytes().to_vec();
              response
            }
            _ => {
                panic!("unrecognized opcode");
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn __execute() -> i32 {
    let mut response = to_arraybuffer_layout(&AuthToken::default().execute().serialize());
    to_ptr(&mut response) + 4
}
