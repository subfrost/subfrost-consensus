use alkanes_runtime::{println, runtime::AlkaneResponder, stdio::stdout};
use alkanes_support::{cellpack::Cellpack, id::AlkaneId, response::CallResponse};
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};
use std::fmt::Write;

#[derive(Default)]
struct LoggerAlkane(());

impl AlkaneResponder for LoggerAlkane {
    fn execute(&self) -> CallResponse {
        let context = self.context().unwrap();
        println!("\nfuel: {}\n", self.fuel());
        println!(
            "executing alkane with id {:?} and caller {:?}",
            context.myself.clone(), context.caller.clone()
        );
        if context.inputs.len() > 0 && context.inputs[0] == 1 {
            let cellpack = Cellpack {
                target: context.myself,
                inputs: vec![],
            };
            println!("running call with cellpack: {:#?}", cellpack);
            let _r = self
                .call(&cellpack, &context.incoming_alkanes, self.fuel())
                .inspect_err(|e| {
                    println!("errored out with: {}", e);
                })
                .unwrap();
            println!("result for call: {:#?}", _r);
        }
        let mut response = CallResponse::default();
        response.data = vec![0x01, 0x02];
        response
    }
}

#[no_mangle]
pub extern "C" fn __execute() -> i32 {
    let mut response = to_arraybuffer_layout(&LoggerAlkane::default().run());
    to_ptr(&mut response) + 4
}
