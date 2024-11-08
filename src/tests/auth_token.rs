use crate::tests::std::alkanes_std_auth_token_build;
use alkanes_support::cellpack::Cellpack;
use alkanes_support::id::AlkaneId;
use anyhow::Result;
use metashrew_support::index_pointer::KeyValuePointer;

use crate::index_block;
use crate::tests::helpers as alkane_helpers;
use crate::tests::std::alkanes_std_owned_token_build;
use alkanes_support::gz::compress;
use metashrew::{clear, index_pointer::IndexPointer, println, stdio::stdout};
use std::fmt::Write;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn test_auth_token() -> Result<()> {
    clear();
    let block_height = 840_000;

    let test_cellpack = Cellpack {
        target: AlkaneId { block: 1, tx: 0 },
        inputs: vec![
            0,    /* opcode (to init new auth token) */
            1,    /* auth_token units */
            1000, /* owned_token token_units */
        ],
    };

    let auth_cellpack = Cellpack {
        target: AlkaneId {
            block: 3,
            tx: 0xffee,
        },
        inputs: vec![100],
    };
    let test_block = alkane_helpers::init_with_multiple_cellpacks_with_tx(
        [
            alkanes_std_auth_token_build::get_bytes(),
            alkanes_std_owned_token_build::get_bytes(),
        ]
        .into(),
        [auth_cellpack, test_cellpack].into(),
    );

    index_block(&test_block, block_height)?;

    println!("block indexed");
    let _auth_token_id_factory = AlkaneId {
        block: 4,
        tx: 0xffee,
    };

    let auth_token_id_deployment = AlkaneId { block: 2, tx: 1 };
    let owned_token_id = AlkaneId { block: 2, tx: 0 };

    println!(
        "balance {:?}",
        IndexPointer::from_keyword("/alkanes/")
            .select(&auth_token_id_deployment.clone().into())
            .keyword("/balances/")
            .select(&owned_token_id.clone().into())
            .get_value::<u128>()
    );

    assert_eq!(
        IndexPointer::from_keyword("/alkanes/")
            .select(&owned_token_id.into())
            .get()
            .as_ref()
            .clone(),
        compress(alkanes_std_owned_token_build::get_bytes().into())?
    );
    assert_eq!(
        IndexPointer::from_keyword("/alkanes/")
            .select(&auth_token_id_deployment.into())
            .get()
            .as_ref()
            .clone(),
        compress(alkanes_std_auth_token_build::get_bytes().into())?
    );

    Ok(())
}
