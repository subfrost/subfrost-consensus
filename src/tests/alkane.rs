#[cfg(test)]
mod tests {
    use crate::tests::std::{alkanes_std_proxy_build, alkanes_std_test_build};
    use std::fmt::{Write};
    use bitcoin::{Block};
    use alkanes_support::cellpack::Cellpack;
    use alkanes_support::id::AlkaneId;
    use alkanes_support::envelope::RawEnvelope;
    use protorune::{view::{protorune_outpoint_to_outpoint_response}, Protorune};

    use crate::tests::helpers as alkane_helpers;
    use bitcoin::OutPoint;
    use hex;
    use metashrew::clear;
    use metashrew_support::utils::{format_key};
    use metashrew::{get_cache, println, stdio::{stdout}};
    use wasm_bindgen_test::wasm_bindgen_test;
    use protorune::test_helpers::{create_block_with_coinbase_tx, get_address, ADDRESS1};
    use crate::tests::std::alkanes_std_owned_token_build;

    use crate::message::AlkaneMessageContext;
    pub fn init_test_with_cellpack(cellpack: Cellpack, wasm_binary: Vec<u8>) -> Block {
        let block_height = 840000;
        let mut test_block = create_block_with_coinbase_tx(block_height);

        let raw_envelope = RawEnvelope::from(wasm_binary);

        let witness = raw_envelope.to_witness();

        // Create a transaction input

        test_block
            .txdata
            .push(alkane_helpers::create_cellpack_with_witness(witness, cellpack));
        test_block
    }

//    #[wasm_bindgen_test]
    /*
    fn std_test_all() {
        clear();
        let block_height = 840_000;

        let test_cellpacks = [
            //create alkane
            Cellpack {
                target: AlkaneId { block: 1, tx: 0 },
                inputs: vec![1],
            },
            /*
            //create second alkane
            Cellpack {
                target: AlkaneId { block: 1, tx: 0 },
                inputs: vec![0],
            },
            //target second alkane to be called with custom opcode
            Cellpack {
                target: AlkaneId { block: 2, tx: 0 },
                inputs: vec![1, 1],
            },
            */
        ];

        let test_block = alkane_helpers::init_test_with_cellpack(test_cellpacks[0].clone());
        let outpoint = OutPoint {
          txid: test_block.txdata[1].txid(),
          vout: 0
        };

        Protorune::index_block::<AlkaneMessageContext>(test_block, block_height as u64).unwrap();
        /*
        get_cache().into_iter().for_each(|(k, v)| {
          if v.len() > 100 {
            ()
          } else {
            println!("{}: {}", format_key(&k.as_ref().to_vec()), hex::encode(v.as_ref()));
            ()
          }
        });
        */
        let result = protorune_outpoint_to_outpoint_response(&outpoint, 1);
        println!("{:?}", result);

    }
    */
    #[wasm_bindgen_test]
    fn std_owned_token() {
        clear();
        let block_height = 840_000;

        let test_cellpacks = [
            //create alkane
            Cellpack {
                target: AlkaneId { block: 1, tx: 0 },
                inputs: vec![0, 1, 1000],
            },
            /*
            //create second alkane
            Cellpack {
                target: AlkaneId { block: 1, tx: 0 },
                inputs: vec![0],
            },
            //target second alkane to be called with custom opcode
            Cellpack {
                target: AlkaneId { block: 2, tx: 0 },
                inputs: vec![1, 1],
            },
            */
        ];

        println!("test!");
        let test_block = init_test_with_cellpack(test_cellpacks[0].clone(), alkanes_std_owned_token_build::get_bytes());
        let outpoint = OutPoint {
          txid: test_block.txdata[1].txid(),
          vout: 0
        };

        Protorune::index_block::<AlkaneMessageContext>(test_block, block_height as u64).unwrap();
        /*
        get_cache().into_iter().for_each(|(k, v)| {
          if v.len() > 100 {
            ()
          } else {
            println!("{}: {}", format_key(&k.as_ref().to_vec()), hex::encode(v.as_ref()));
            ()
          }
        });
        */
        let result = protorune_outpoint_to_outpoint_response(&outpoint, 1);
        println!("{:?}", result);

    }

    /*
    #[wasm_bindgen_test]
    async fn protomessage_with_binary_test() {
        clear();
        let input_cellpack = Cellpack {
            target: AlkaneId { block: 1, tx: 0 },
            inputs: vec![
                123456789123456789123456789u128,
                987654321987654321987654321u128,
            ],
        };

        let test_block = alkane_helpers::init_test_with_cellpack(input_cellpack);

        assert!(
            Protorune::index_block::<AlkaneMessageContext>(test_block.clone(), 840000 as u64)
                .is_ok()
        );

        // TODO: Fix protomessage refunding. this does not work rn
        // // tx 0 is coinbase, tx 1 is runestone
        // let outpoint_address: OutPoint = OutPoint {
        //     txid: test_block.txdata[1].txid(),
        //     vout: 0,
        // };
        // // check runes balance
        // let sheet = load_sheet(
        //     &tables::RUNES
        //         .OUTPOINT_TO_RUNES
        //         .select(&consensus_encode(&outpoint_address).unwrap()),
        // );

        // let protorunes_sheet = load_sheet(
        //     &tables::RuneTable::for_protocol(protocol_id.into())
        //         .OUTPOINT_TO_RUNES
        //         .select(&consensus_encode(&outpoint_address).unwrap()),
        // );

        // let protorune_id = ProtoruneRuneId {
        //     block: block_height as u128,
        //     tx: 1,
        // };
        // let stored_balance_address = sheet.get(&protorune_id);
        // assert_eq!(stored_balance_address, 0);

        // let stored_protorune_balance = protorunes_sheet.get(&protorune_id);
        // assert_eq!(stored_protorune_balance, 1000);
    }
    */
}
