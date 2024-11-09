use alkanes_runtime::{
    println,
    runtime::AlkaneResponder,
    stdio::{stdout, Write},
    storage::StoragePointer,
    token::Token,
};
use alkanes_support::utils::shift;
use alkanes_support::{parcel::AlkaneTransfer, response::CallResponse};
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};
use metashrew_support::index_pointer::KeyValuePointer;
use std::sync::Arc;

#[derive(Default)]
pub struct AuthToken(());

impl Token for AuthToken {
    fn name(&self) -> String {
        String::from("AUTH")
    }
    fn symbol(&self) -> String {
        String::from("AUTH")
    }
}

impl AlkaneResponder for AuthToken {
    fn execute(&self) -> CallResponse {
        let context = self.context().unwrap();
        let mut inputs = context.inputs.clone();
        let mut response: CallResponse = CallResponse::forward(&context.incoming_alkanes.clone());
        match shift(&mut inputs).unwrap() {
            0 => {
                println!("initializing auth token");
                let mut pointer = StoragePointer::from_keyword("/initialized");
                if pointer.get().len() == 0 {
                    let amount = shift(&mut inputs).unwrap();
                    response.alkanes = context.incoming_alkanes.clone();
                    response.alkanes.0.push(AlkaneTransfer {
                        id: context.myself.clone(),
                        value: amount,
                    });
                    println!("intialize response: {:?}", response);
                    pointer.set(Arc::new(vec![0x01]));
                    response
                } else {
                    panic!("already initialized");
                }
            }
            1 => {
                if context.incoming_alkanes.0.len() != 1 {
                    panic!("did not authenticate with only the authentication token");
                }
                let transfer = context.incoming_alkanes.0[0].clone();
                if transfer.id != context.myself.clone() {
                    panic!("supplied alkane is not authentication token");
                }
                if transfer.value < 1 {
                    panic!("less than 1 unit of authentication token supplied to authenticate");
                }
                response.data = vec![0x01];
                response.alkanes.0.push(transfer);
                response
            }
            99 => {
                response.data = self.name().into_bytes().to_vec();
                response
            }
            100 => {
                response.data = self.symbol().into_bytes().to_vec();
                response
            }
            _ => {
                panic!("unrecognized opcode");
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn __execute() -> i32 {
    let mut response = to_arraybuffer_layout(&AuthToken::default().run());
    to_ptr(&mut response) + 4
}
