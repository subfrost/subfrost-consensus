use alkanes_runtime::runtime::AlkaneResponder;
use alkanes_support::response::CallResponse;
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};

#[derive(Default)]
pub struct RefunderAlkane(());

impl AlkaneResponder for RefunderAlkane {
    fn execute(&self) -> CallResponse {
        let context = self.context().unwrap();
        CallResponse::forward(&context.incoming_alkanes)
    }
}

#[no_mangle]
pub extern "C" fn __execute() -> i32 {
    let mut response = to_arraybuffer_layout(&RefunderAlkane::default().execute().serialize());
    to_ptr(&mut response) + 4
}
