use crate::tests::std::{alkanes_std_amm_pool_build, alkanes_std_auth_token_build};
use alkanes_support::cellpack::Cellpack;
use alkanes_support::id::AlkaneId;
use anyhow::Result;

use crate::index_block;
use crate::tests::helpers as alkane_helpers;
use crate::tests::std::alkanes_std_owned_token_build;
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
                tx: 0xffee,
            },
            inputs: vec![100],
        },
        // token 1 init and mint
        Cellpack {
            target: AlkaneId { block: 1, tx: 0 },
            inputs: vec![0, 1, 1000],
        },
        // token 2 init and mint
        Cellpack {
            target: AlkaneId { block: 1, tx: 0 },
            inputs: vec![0, 1, 1000],
        },
        //pool init
        Cellpack {
            target: AlkaneId { block: 1, tx: 0 },
            inputs: vec![0, 2, 0, 2, 1],
        },
    ]
    .into();
    let test_block = alkane_helpers::init_with_multiple_cellpacks_with_tx(
        [
            alkanes_std_auth_token_build::get_bytes(),
            alkanes_std_owned_token_build::get_bytes(),
            alkanes_std_owned_token_build::get_bytes(),
            alkanes_std_amm_pool_build::get_bytes(),
        ]
        .into(),
        cellpacks,
    );

    index_block(&test_block, block_height)?;
    Ok(())
}
