use crate::tests::std::{alkanes_std_amm_pool_build, alkanes_std_auth_token_build};
use alkanes_support::cellpack::Cellpack;
use alkanes_support::id::AlkaneId;
use anyhow::Result;
use protorune::{balance_sheet::load_sheet, message::MessageContext, tables::RuneTable};
use metashrew_support::index_pointer::{KeyValuePointer};
use protorune_support::utils::{consensus_encode};
use alkanes::message::{AlkaneMessageContext};
use bitcoin::blockdata::transaction::{OutPoint};

use crate::index_block;
use crate::tests::helpers as alkane_helpers;
use crate::tests::std::{alkanes_std_owned_token_build, alkanes_std_amm_factory_build};
#[allow(unused_imports)]
use metashrew::{
    clear,
    index_pointer::IndexPointer,
    println,
    stdio::{stdout, Write},
};
use wasm_bindgen_test::wasm_bindgen_test;
#[wasm_bindgen_test]
fn test_amm_pool() -> Result<()> {
    clear();
    let block_height = 840_000;
    let cellpacks: Vec<Cellpack> = [
        //auth token factory init
        Cellpack {
            target: AlkaneId {
                block: 3,
                tx: 0xffef,
            },
            inputs: vec![50],
        },
        Cellpack {
            target: AlkaneId {
                block: 3,
                tx: 0xffee
            },
            inputs: vec![100]
        },
        // token 1 init and mint
        Cellpack {
            target: AlkaneId { block: 1, tx: 0 },
            inputs: vec![0],
        },
        Cellpack {
            target: AlkaneId { block: 1, tx: 0 },
            inputs: vec![0, 1, 1000],
        },
        Cellpack {
            target: AlkaneId { block: 5, tx: 1 },
            inputs: vec![0, 1, 1000]
        }
    ]
    .into();
    let test_block = alkane_helpers::init_with_multiple_cellpacks_with_tx(
        [
            alkanes_std_amm_pool_build::get_bytes(),
            alkanes_std_auth_token_build::get_bytes(),
            alkanes_std_amm_factory_build::get_bytes(),
            alkanes_std_owned_token_build::get_bytes(),
            [].into()
        ]
        .into(),
        cellpacks,
    );
    let len = test_block.txdata.len();
    let outpoint = OutPoint {
      txid: test_block.txdata[len - 1].compute_txid(),
      vout: 0
    };
    let sheet = load_sheet(
        &RuneTable::for_protocol(AlkaneMessageContext::protocol_tag())
            .OUTPOINT_TO_RUNES
            .select(&consensus_encode(&outpoint)?),
    );
    println!("balances at end {:?}", sheet);
    

    index_block(&test_block, block_height)?;
    Ok(())
}
