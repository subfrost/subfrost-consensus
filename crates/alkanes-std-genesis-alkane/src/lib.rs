use alkanes_runtime::{runtime::AlkaneResponder, storage::StoragePointer, token::Token};
use alkanes_support::utils::overflow_error;
use alkanes_support::{
    context::Context, parcel::AlkaneTransfer, response::CallResponse, utils::shift,
};
use anyhow::{anyhow, Result};
use bitcoin::hashes::Hash;
use bitcoin::Block;
use hex;
use metashrew_support::block::AuxpowBlock;
use metashrew_support::compat::{to_arraybuffer_layout, to_passback_ptr};
use metashrew_support::index_pointer::KeyValuePointer;
use std::io::Cursor;
pub mod chain;
use crate::chain::{ChainConfiguration, CONTEXT_HANDLE};

#[derive(Default)]
pub struct GenesisAlkane(());

impl Token for GenesisAlkane {
    fn name(&self) -> String {
        String::from("{TBD}")
    }
    fn symbol(&self) -> String {
        String::from("{TBD}")
    }
}

#[cfg(feature = "regtest")]
impl ChainConfiguration for GenesisAlkane {
    fn block_reward(&self, n: u64) -> u128 {
        return (50e8 as u128) / (1u128 << ((n as u128) / 210000u128));
    }
    fn genesis_block(&self) -> u64 {
        0
    }
    fn average_payout_from_genesis(&self) -> u128 {
        50_000_000
    }
    fn total_supply(&self) -> u128 {
        131250000000000
    }
}

#[cfg(not(feature = "regtest"))]
impl ChainConfiguration for GenesisAlkane {
    fn block_reward(&self, n: u64) -> u128 {
        return (50e8 as u128) / (1u128 << ((n as u128) / 210000u128));
    }
    fn genesis_block(&self) -> u64 {
        840000
    }
    fn average_payout_from_genesis(&self) -> u128 {
        312500000
    }
    fn total_supply(&self) -> u128 {
        131250000000000
    }
}

impl GenesisAlkane {
    fn block(&self) -> Result<Block> {
        Ok(AuxpowBlock::parse(&mut Cursor::<Vec<u8>>::new(CONTEXT_HANDLE.block()))?.to_consensus())
    }
    pub fn seen_pointer(&self, hash: &Vec<u8>) -> StoragePointer {
        StoragePointer::from_keyword("/seen/").select(&hash)
    }
    pub fn hash(&self, block: &Block) -> Vec<u8> {
        block.block_hash().as_byte_array().to_vec()
    }
    pub fn total_supply_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/totalsupply")
    }
    pub fn total_supply(&self) -> u128 {
        self.total_supply_pointer().get_value::<u128>()
    }
    pub fn increase_total_supply(&self, v: u128) -> Result<()> {
        self.set_total_supply(overflow_error(self.total_supply().checked_add(v))?);
        Ok(())
    }
    pub fn set_total_supply(&self, v: u128) {
        self.total_supply_pointer().set_value::<u128>(v);
    }
    pub fn observe_mint(&self, block: &Block) -> Result<()> {
        let hash = self.hash(block);
        let mut pointer = self.seen_pointer(&hash);
        if pointer.get().len() == 0 {
            pointer.set_value::<u32>(1);
            Ok(())
        } else {
            Err(anyhow!(format!(
                "already minted for block {}",
                hex::encode(&hash)
            )))
        }
    }
    pub fn mint(&self, context: &Context) -> Result<AlkaneTransfer> {
        self.observe_mint(&self.block()?)?;
        let value = self.current_block_reward();
        let mut total_supply_pointer = self.total_supply_pointer();
        let total_supply = total_supply_pointer.get_value::<u128>();
        if total_supply >= self.total_supply() {
            return Err(anyhow!("total supply has been reached"));
        }
        total_supply_pointer.set_value::<u128>(total_supply + value);
        Ok(AlkaneTransfer {
            id: context.myself.clone(),
            value,
        })
    }
    pub fn observe_initialization(&self) -> Result<()> {
        self.observe_mint(&self.block()?)?;
        let mut initialized_pointer = StoragePointer::from_keyword("/initialized");
        if initialized_pointer.get().len() == 0 {
            initialized_pointer.set_value::<u32>(1);
            Ok(())
        } else {
            Err(anyhow!("already initialized"))
        }
    }
}

impl AlkaneResponder for GenesisAlkane {
    fn execute(&self) -> CallResponse {
        let context = self.context().unwrap();
        let mut inputs = context.inputs.clone();
        let mut response = CallResponse::forward(&context.incoming_alkanes);
        match shift(&mut inputs).unwrap() {
            0 => {
                self.observe_initialization().unwrap();
                let premine = self.premine().unwrap();
                response.alkanes.0.push(AlkaneTransfer {
                    id: context.myself.clone(),
                    value: premine,
                });
                self.increase_total_supply(premine).unwrap();
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
        &GenesisAlkane::default().run(),
    )));
    to_passback_ptr(response)
}
