use alkanes_support::response::CallResponse;
use alkanes_runtime::{
    runtime::AlkaneResponder,
    println,
    stdio::{stdout},
};
use std::fmt::Write;
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
pub extern "C" fn __execute() -> i32 {
    let mut response = to_arraybuffer_layout(&LoggerAlkane::default().execute().serialize());
    to_ptr(&mut response) + 4
}
