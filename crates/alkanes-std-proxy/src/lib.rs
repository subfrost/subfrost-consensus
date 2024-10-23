use alkanes_runtime::runtime::AlkaneResponder;
use alkanes_support::response::CallResponse;
use bitcoin::blockdata::transaction::Transaction;
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};
use protorune_support::utils::consensus_decode;

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
        consensus_decode::<Transaction>(&mut std::io::Cursor::new(self.transaction())).unwrap();
        let mut inputs = context.inputs.clone();
        let _opcode = shift(&mut inputs).unwrap();
        CallResponse::default()
    }
}

#[no_mangle]
pub extern "C" fn __execute() -> i32 {
    let mut response = to_arraybuffer_layout(&Proxy::default().execute().serialize());
    to_ptr(&mut response) + 4
}
