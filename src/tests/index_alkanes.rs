#[cfg(test)]
mod tests {
    use protorune::proto::protorune::{RunesByHeightRequest, WalletRequest};

    use protorune::test_helpers as helpers;
    use protorune::test_helpers::{
        create_test_transaction_with_witness, display_list_as_hex, display_vec_as_hex,
    };
    use protorune::Protorune;
    use protorune::{tables, view};

    use bitcoin::consensus::serialize;
    use bitcoin::hashes::Hash;
    use hex;
    use std::fmt::Write;

    use metashrew::{clear, index_pointer::KeyValuePointer};

    use protobuf::{Message, SpecialFields};

    use crate::message::AlkaneMessageContext;
    use std::str::FromStr;
    use std::sync::Arc;
    use crate::tests::alkanes_std_test_build;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_data_unwrap() -> anyhow::Result<()> {
        clear();
        let (mut test_block, _) = helpers::create_block_with_rune_tx();
        let wasm_binary = alkanes_std_test_build::get_bytes();
        let tx = create_test_transaction_with_witness(wasm_binary);
        test_block.txdata.push(tx);
        let _ = Protorune::index_block::<AlkaneMessageContext>(test_block.clone(), 840001);
        let req = (WalletRequest {
            wallet: "bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu"
                .as_bytes()
                .to_vec(),
            special_fields: SpecialFields::new(),
        })
        .write_to_bytes()
        .unwrap();
        // let test_val = view::runes_by_address(&req).unwrap();
        // let runes: Vec<protorune::proto::protorune::OutpointResponse> = test_val.clone().outpoints;
        // assert_eq!(runes[0].height, 840001);
        // assert_eq!(runes[0].txindex, 0);
        Ok(())
    }
}
