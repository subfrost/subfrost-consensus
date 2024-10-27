use alkanes_runtime::{println, runtime::AlkaneResponder, stdio::stdout};
use alkanes_support::{cellpack::Cellpack, id::AlkaneId, response::CallResponse};
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};
use std::fmt::Write;

#[derive(Default)]
struct LoggerAlkane(());

impl AlkaneResponder for LoggerAlkane {
    fn execute(&self) -> CallResponse {
        let context = self.context().unwrap();
        println!(
            "executing alkane with id {:?} and caller {:?}",
            context.myself, context.caller
        );
        println!("{:#?}", context.inputs);
        if context.inputs.len() > 0 && context.inputs[0] == 1 {
            let cellpack = Cellpack {
                target: AlkaneId {
                    block: 2,
                    tx: context.inputs[1],
                },
                inputs: vec![0],
            };
            let _r = self
                .call(&cellpack, &context.incoming_alkanes, 500)
                .is_err_and(|e| {
                    println!("{}", e);
                    true
                });
        }
        CallResponse::default()
    }
}

#[no_mangle]
pub extern "C" fn __execute() -> i32 {
    let mut response = to_arraybuffer_layout(&LoggerAlkane::default().execute().serialize());
    to_ptr(&mut response) + 4
}
