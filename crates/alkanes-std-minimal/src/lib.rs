use alkanes_runtime::runtime::AlkaneResponder;
use alkanes_support::response::CallResponse;
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};

#[derive(Default)]
pub struct MinimalExample(());

impl AlkaneResponder for MinimalExample {
    fn execute(&self) -> CallResponse {
        CallResponse::default()
    }
}

#[no_mangle]
pub extern "C" fn __execute() -> i32 {
    let mut response = to_arraybuffer_layout(&MinimalExample::default().run());
    to_ptr(&mut response) + 4
}
