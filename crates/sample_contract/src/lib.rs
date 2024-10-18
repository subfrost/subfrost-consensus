use alkanes_runtime::{
    response::CallResponse,
    runtime::AlkaneResponder,
    stdio::{println, stdout},
};
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};

#[derive(Default)]
struct LoggerAlkane(());

impl AlkaneResponder for LoggerAlkane {
    fn execute(&self) -> CallResponse {
        println!("hello world!");
        CallResponse::default()
    }
}

#[no_mangle]
pub extern "C" fn __execute(x: i32) -> i32 {
    let response = to_arraybuffer_layout(&LoggerAlkane::default().execute().serialize());
    to_ptr(&mut response) + 4
}
