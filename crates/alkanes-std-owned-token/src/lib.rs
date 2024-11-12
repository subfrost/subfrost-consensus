use alkanes_runtime::{auth::AuthenticatedResponder, token::Token};
use alkanes_runtime::{
    println,
    stdio::{stdout, Write},
};
use alkanes_runtime::{runtime::AlkaneResponder, storage::StoragePointer};
use alkanes_support::utils::shift;
use alkanes_support::{context::Context, parcel::AlkaneTransfer, response::CallResponse};
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};
use metashrew_support::index_pointer::KeyValuePointer;
use std::sync::Arc;

#[derive(Default)]
pub struct OwnedToken(());

pub trait MintableToken: Token {
    fn mint(&self, context: &Context, value: u128) -> AlkaneTransfer {
        AlkaneTransfer {
            id: context.myself.clone(),
            value,
        }
    }
}

impl Token for OwnedToken {
    fn name(&self) -> String {
        String::from("EXAMPLE")
    }
    fn symbol(&self) -> String {
        String::from("EXAMPLE")
    }
}
impl MintableToken for OwnedToken {}

impl AuthenticatedResponder for OwnedToken {}

impl AlkaneResponder for OwnedToken {
    fn execute(&self) -> CallResponse {
        let context = self.context().unwrap();
        let mut inputs = context.inputs.clone();
        let mut response: CallResponse = CallResponse::forward(&context.incoming_alkanes.clone());
        println!("incoming alkanes in owned token: {:?}", response);
        match shift(&mut inputs).unwrap() {
            0 => {
                println!("initializing std token");
                let mut pointer = StoragePointer::from_keyword("/initialized");
                if pointer.get().len() == 0 {
                    let auth_token_units = shift(&mut inputs).unwrap();
                    let token_units = shift(&mut inputs).unwrap();
                    println!("deploy auth token");
                    response
                        .alkanes
                        .0
                        .push(self.deploy_auth_token(auth_token_units).unwrap());
                    response.alkanes.0.push(AlkaneTransfer {
                        id: context.myself.clone(),
                        value: token_units,
                    });
                    pointer.set(Arc::new(vec![0x01]));
                    println!("deployed");
                    response
                } else {
                    panic!("already initialized");
                }
            }
            1 => {
                self.only_owner().unwrap();
                let token_units = shift(&mut inputs).unwrap();
                let transfer = self.mint(&context, token_units);
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
    let mut response = to_arraybuffer_layout(&OwnedToken::default().run());
    to_ptr(&mut response) + 4
}
