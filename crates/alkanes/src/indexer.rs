use ordinals::{Artifact, Runestone};
use protorune::{message::MessageContext, Protorune};
use crate::message::AlkaneMessageContext;
use crate::vm::fuel::set_message_count;
use crate::network::{genesis, is_genesis};
use protorune_support::protostone::{Protostone};
use bitcoin::blockdata::block::{Block};
use anyhow::{Result};

pub fn index_block(block: &Block, height: u32) -> Result<()> {
    if is_genesis(height.into()) {
        genesis(&block).unwrap();
    }
    count_alkanes_protomessages(&block);
    Protorune::index_block::<AlkaneMessageContext>(block.clone(), height.into())?;
    Ok(())
}

pub fn count_alkanes_protomessages(block: &Block) {
    let mut count: u64 = 0;
    for tx in &block.txdata {
        if let Some(Artifact::Runestone(ref runestone)) = Runestone::decipher(tx) {
            if let Ok(protostones) = Protostone::from_runestone(runestone) {
                for protostone in protostones {
                    if protostone.protocol_tag == AlkaneMessageContext::protocol_tag()
                        && protostone.message.len() != 0
                    {
                        count = count + 1;
                    }
                }
            }
        }
    }
    set_message_count(count);
}
