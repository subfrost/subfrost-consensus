use alkanes_runtime::{println, runtime::AlkaneResponder, stdio::stdout};
use alkanes_support::{id::AlkaneId, response::CallResponse};
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};
use bitcoin::blockdata::{Transaction};
use alkanes_support::envelope::{RawEnvelope};
use std::fmt::Write;

#[derive(Default)]
struct Proxy(());

fn shift<T>(v: &mut Vec<T>) -> Option<T> {
  if v.is_empty() {
    None
  } else {
    Some(v.remove(0))
  }
  
}

impl AlkaneResponder for Proxy {
    fn execute(&self) -> CallResponse {
        let context = self.context().unwrap();
        Transaction::from_bytes(&self.transaction());
        let mut inputs = context.inputs.clone();
        let opcode = shift(&mut inputs).unwrap();
        CallResponse::default()
    }
}

#[no_mangle]
pub extern "C" fn __execute() -> i32 {
    let mut response = to_arraybuffer_layout(&Proxy::default().execute().serialize());
    to_ptr(&mut response) + 4
}
