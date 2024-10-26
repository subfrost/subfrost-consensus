use alkanes_runtime::{println, runtime::AlkaneResponder, stdio::stdout};
use alkanes_support::{id::AlkaneId, response::CallResponse};
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};
use std::fmt::Write;

#[derive(Default)]
struct LoggerAlkane(());

impl AlkaneResponder for LoggerAlkane {
    fn execute(&self) -> CallResponse {
        let context = self.context().unwrap();
        context
            .incoming_alkanes
            .0
            .into_iter()
            .map(|parcel| {
                println!("{}", parcel);
            })
            .for_each(drop);
        println!(
            "hello world! {}",
            self.balance(
                &AlkaneId {
                    block: 100,
                    tx: 100,
                },
                &AlkaneId {
                    block: 100,
                    tx: 100,
                },
            )
        );
        CallResponse::default()
    }
}

#[no_mangle]
pub extern "C" fn __execute() -> i32 {
    let mut response = to_arraybuffer_layout(&LoggerAlkane::default().execute().serialize());
    to_ptr(&mut response) + 4
}
