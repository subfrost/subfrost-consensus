use alkanes_runtime::{token::{Token}, storage::{StoragePointer}, runtime::{AlkaneResponder}};
use alkanes_support::{parcel::AlkaneTransfer, response::CallResponse, utils::shift};
use metashrew_support::compat::{to_passback_ptr, to_arraybuffer_layout};
use metashrew_support::index_pointer::{KeyValuePointer};
use hex_lit::{hex};
use anyhow::{anyhow, Result};

#[derive(Default)]
pub struct Orbital(());

impl Token for Orbital {
  fn name(&self) -> String {
    String::from("NFT")
  }
  fn symbol(&self) -> String {
    String::from("NFT")
  }
}

impl Orbital {
  pub fn total_supply_pointer(&self) -> StoragePointer {
    StoragePointer::from_keyword("/totalsupply")
  }
  pub fn total_supply(&self) -> u128 {
    self.total_supply_pointer().get_value::<u128>()
  }
  pub fn set_total_supply(&self, v: u128) {
    self.total_supply_pointer().set_value::<u128>(v);
  }
  pub fn observe_initialization(&self) -> Result<()> {
    let mut initialized_pointer = StoragePointer::from_keyword("/initialized");
    if initialized_pointer.get().len() == 0 {
      initialized_pointer.set_value::<u32>(1);
      Ok(())
    } else {
      Err(anyhow!("already initialized"))
    }
  }
  pub fn data(&self) -> Vec<u8> {
    // in this reference implementation, we return a 1x1 PNG
    // NFT data can be anything, however
    (&hex!("89504e470d0a1a0a0000000d494844520000000100000001010300000025db56ca00000003504c5445000000a77a3dda0000000174524e530040e6d8660000000a4944415408d76360000000020001e221bc330000000049454e44ae426082")).to_vec()
  }
}

impl AlkaneResponder for Orbital {
    fn execute(&self) -> CallResponse {
        let context = self.context().unwrap();
        let mut inputs = context.inputs.clone();
        let mut response = CallResponse::forward(&context.incoming_alkanes);
        match shift(&mut inputs).unwrap() {
          0 => {
            self.observe_initialization().unwrap();
            self.set_total_supply(1);
            response.alkanes.0.push(AlkaneTransfer {
              id: context.myself.clone(),
              value: 1u128
            });
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
          1000 => {
            response.data = self.data()
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
    let mut response = to_arraybuffer_layout(&Orbital::default().run());
    to_passback_ptr(&mut response)
}
