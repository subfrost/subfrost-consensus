use alkanes_runtime::{runtime::AlkaneResponder, storage::StoragePointer, token::Token};
use alkanes_support::{
    context::Context, parcel::AlkaneTransfer, response::CallResponse, utils::shift, id::{AlkaneId}
};
use anyhow::{Result};
use metashrew_support::compat::{to_arraybuffer_layout, to_passback_ptr};
use metashrew_support::index_pointer::KeyValuePointer;

#[derive(Default)]
pub struct GenesisProtorune(());

impl Token for GenesisProtorune {
    fn name(&self) -> String {
        String::from("Genesis Protorune")
    }
    fn symbol(&self) -> String {
        String::from("aGP")
    }
}

impl GenesisProtorune {
    pub fn total_supply_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/totalsupply")
    }
    pub fn total_supply(&self) -> u128 {
        self.total_supply_pointer().get_value::<u128>()
    }
    pub fn set_total_supply(&self, v: u128) {
        self.total_supply_pointer().set_value::<u128>(v);
    }
    pub fn mint(&self, context: &Context) -> Result<AlkaneTransfer> {
        if context.incoming_alkanes.0.len() != 1 || &context.incoming_alkanes.0[0].id != &(AlkaneId { block: 849236, tx: 298 }) {
          panic!("can only mint in response to incoming QUORUM•GENESIS•PROTORUNE");
        }
        let value = context.incoming_alkanes.0[0].value;
        let mut total_supply_pointer = self.total_supply_pointer();
        total_supply_pointer.set_value::<u128>(total_supply_pointer.get_value::<u128>() + value);
        Ok(AlkaneTransfer {
            id: context.myself.clone(),
            value,
        })
    }
}

impl AlkaneResponder for GenesisProtorune {
    fn execute(&self) -> CallResponse {
        let context = self.context().unwrap();
        let mut inputs = context.inputs.clone();
        let mut response = CallResponse::forward(&context.incoming_alkanes);
        match shift(&mut inputs).unwrap() {
            0 => {
                // no initialization logic
            }
            77 => {
                response.alkanes.0.push(self.mint(&context).unwrap());
            }
            99 => {
                response.data = self.name().into_bytes().to_vec();
            }
            100 => {
                response.data = self.symbol().into_bytes().to_vec();
            }
            101 => {
                response.data = (&self.total_supply().to_le_bytes()).to_vec();
            }
            _ => {
                panic!("unrecognized opcode");
            }
        }
        response
    }
}

#[no_mangle]
pub extern "C" fn __execute() -> i32 {
    let response: &'static mut Vec<u8> = Box::leak(Box::new(to_arraybuffer_layout(
        &GenesisProtorune::default().run(),
    )));
    to_passback_ptr(response)
}
