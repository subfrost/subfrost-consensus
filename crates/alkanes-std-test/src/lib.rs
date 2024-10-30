use alkanes_support::{cellpack::Cellpack, id::AlkaneId, response::CallResponse};
//use alkanes_runtime::{println, stdio::{stdout}, runtime::{AlkaneResponder}};
use alkanes_runtime::runtime::AlkaneResponder;
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};
use std::fmt::Write;

#[derive(Default)]
struct LoggerAlkane(());

impl AlkaneResponder for LoggerAlkane {
    fn execute(&self) -> CallResponse {
        let context = self.context().unwrap();
        println!("incoming_alkanes: {:?}", context.incoming_alkanes);
        if context.inputs.len() > 0 && context.inputs[0] == 1 {
            let cellpack = Cellpack {
                target: context.myself,
                inputs: vec![],
            };
            let _r = self
                .call(&cellpack, &context.incoming_alkanes, self.fuel())
                .unwrap();
            ()
        } else {
            ()
        }
        let mut response = CallResponse::forward(&context.incoming_alkanes);
        response.data = vec![0x01, 0x02];
        response
    }
}

#[no_mangle]
pub extern "C" fn __execute() -> i32 {
    let mut response = to_arraybuffer_layout(&LoggerAlkane::default().run());
    to_ptr(&mut response) + 4
}
