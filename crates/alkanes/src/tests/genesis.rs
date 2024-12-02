use alkanes::message::AlkaneMessageContext;
use alkanes_support::cellpack::Cellpack;
use alkanes_support::id::AlkaneId;
use anyhow::Result;
use bitcoin::blockdata::transaction::OutPoint;
use metashrew_support::index_pointer::KeyValuePointer;
use protorune::{balance_sheet::load_sheet, message::MessageContext, tables::RuneTable};

use protorune_support::utils::consensus_encode;

use crate::index_block;
use crate::tests::helpers as alkane_helpers;
use crate::tests::std::alkanes_std_genesis_alkane_build;
#[allow(unused_imports)]
use metashrew::{clear, get_cache, index_pointer::IndexPointer, println, stdio::stdout};
use std::fmt::Write;
use wasm_bindgen_test::wasm_bindgen_test;
#[wasm_bindgen_test]
fn test_genesis() -> Result<()> {
    clear();
    let block_height = 850_000;
    let cellpacks: Vec<Cellpack> = [
        //auth token factory init
        Cellpack {
            target: AlkaneId { block: 1, tx: 0 },
            inputs: vec![0],
        },
    ]
    .into();
    let test_block = alkane_helpers::init_with_multiple_cellpacks_with_tx(
        [alkanes_std_genesis_alkane_build::get_bytes(), vec![]].into(),
        cellpacks,
    );
    let len = test_block.txdata.len();
    let outpoint = OutPoint {
        txid: test_block.txdata[len - 1].compute_txid(),
        vout: 0,
    };
    println!(
        "runestone: {}",
        hex::encode(&test_block.txdata[1].output[1].script_pubkey)
    );

    index_block(&test_block, block_height)?;
    let cellpacks2 = vec![Cellpack {
        target: AlkaneId { block: 2, tx: 0 },
        inputs: vec![77],
    }];
    let test_block2 = alkane_helpers::init_with_multiple_cellpacks_with_tx([].into(), cellpacks2);
    index_block(&test_block2, block_height + 1)?;
    let ptr = RuneTable::for_protocol(AlkaneMessageContext::protocol_tag())
        .OUTPOINT_TO_RUNES
        .select(&consensus_encode(&outpoint)?);
    let sheet = load_sheet(&ptr);
    /*
    get_cache().iter().for_each(|(k, v)| {
      if v.len() < 300 { println!("{}: {}", format_key(&k.as_ref().clone()), hex::encode(&v.as_ref().clone())); }
    });
    */
    println!("balances at end {:?}", sheet);
    Ok(())
}
