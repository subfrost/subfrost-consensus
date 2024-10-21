#[cfg(test)]
mod tests {
    use alkanes_support::cellpack::Cellpack;
    use alkanes_support::id::AlkaneId;
    use bitcoin::Transaction;
    use bitcoin::{
        address::NetworkChecked, Address, Amount, OutPoint, ScriptBuf, Sequence, TxIn, TxOut,
        Witness,
    };
    use metashrew::index_pointer::KeyValuePointer;
    use protorune::balance_sheet::load_sheet;
    use protorune::protostone::{Protostone, Protostones};
    use protorune::test_helpers::{self as helpers, get_address, ADDRESS1};
    use protorune::{tables, Protorune};
    use protorune_support::balance_sheet::{BalanceSheet, ProtoruneRuneId};
    use protorune_support::utils::consensus_encode;

    use crate::tests::helpers as alkane_helpers;
    use bitcoin::secp256k1::PublicKey;
    use metashrew::{clear, get_cache, println, stdio::stdout};
    use metashrew_support::utils::format_key;
    use ordinals::{Etching, Rune, Runestone};
    use std::fmt::Write;
    use std::str::FromStr;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::envelope::RawEnvelope;
    use crate::message::AlkaneMessageContext;

    use crate::tests::std::alkanes_std_test_build;

    pub fn print_cache() {
        let cache = get_cache();

        for (key, value) in cache.iter() {
            let formatted_key = format_key(key);
            let formatted_value = format_key(value);

            println!("{}: {}", formatted_key, formatted_value);
        }
    }

    /// In one runestone, etches a rune, then protoburns it
    #[wasm_bindgen_test]
    fn protoburn_test() {
        clear();
        let block_height = 840000;
        let protocol_id = 1;
        let mut test_block = helpers::create_block_with_coinbase_tx(block_height);

        let previous_output = OutPoint {
            txid: bitcoin::Txid::from_str(
                "0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap(),
            vout: 0,
        };
        let input_script = ScriptBuf::new();

        let protoburn_tx = helpers::create_protoburn_transaction(previous_output, protocol_id);

        test_block.txdata.push(protoburn_tx);
        assert!(Protorune::index_block::<AlkaneMessageContext>(
            test_block.clone(),
            block_height as u64
        )
        .is_ok());
        /*
        get_cache().iter().for_each(|(k, v)| {
          println!("{}: {}", format_key(k.as_ref()), hex::encode(v.as_ref()));
        });
        */

        // tx 0 is coinbase, tx 1 is runestone
        let outpoint_address: OutPoint = OutPoint {
            txid: test_block.txdata[1].txid(),
            vout: 0,
        };
        // check runes balance
        let sheet = load_sheet(
            &tables::RUNES
                .OUTPOINT_TO_RUNES
                .select(&consensus_encode(&outpoint_address).unwrap()),
        );

        let protorunes_sheet = load_sheet(
            &tables::RuneTable::for_protocol(protocol_id.into())
                .OUTPOINT_TO_RUNES
                .select(&consensus_encode(&outpoint_address).unwrap()),
        );

        // print_cache();

        let protorune_id = ProtoruneRuneId {
            block: block_height as u128,
            tx: 1,
        };
        // let v: Vec<u8> = protorune_id.into();
        let stored_balance_address = sheet.get(&protorune_id);
        assert_eq!(stored_balance_address, 0);
        let stored_protorune_balance = protorunes_sheet.get(&protorune_id);
        assert_eq!(stored_protorune_balance, 1000);
    }

    #[wasm_bindgen_test]
    fn protomessage_no_binary_test() {
        clear();
        let block_height = 840000;
        let protocol_id = 122;
        let mut test_block = helpers::create_block_with_coinbase_tx(block_height);

        let previous_output = OutPoint {
            txid: bitcoin::Txid::from_str(
                "0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap(),
            vout: 0,
        };
        let input_script = ScriptBuf::new();

        let txin = TxIn {
            previous_output,
            script_sig: input_script,
            sequence: Sequence::MAX,
            witness: Witness::new(),
        };

        let address: Address<NetworkChecked> = get_address(&ADDRESS1);

        let script_pubkey = address.script_pubkey();

        let txout = TxOut {
            value: Amount::from_sat(100_000_000).to_sat(),
            script_pubkey,
        };

        let runestone: ScriptBuf = (Runestone {
            etching: Some(Etching {
                divisibility: Some(2),
                premine: Some(1000),
                rune: Some(Rune::from_str("TESTTESTTEST").unwrap()),
                spacers: Some(0),
                symbol: Some(char::from_str("A").unwrap()),
                turbo: true,
                terms: None,
            }),
            pointer: Some(1), // points to the OP_RETURN, so therefore targets the protoburn
            edicts: Vec::new(),
            mint: None,
            protocol: match vec![
                Protostone {
                    burn: Some(protocol_id),
                    edicts: vec![],
                    pointer: Some(4),
                    refund: None,
                    from: None,
                    protocol_tag: 13, // this value must be 13 if protoburn
                    message: vec![],
                },
                Protostone {
                    // protomessage with nonsensical inforamtion, which should all be refunded
                    message: Cellpack {
                        target: AlkaneId { block: 1, tx: 0 },
                        inputs: vec![
                            123456789123456789123456789u128,
                            987654321987654321987654321u128,
                        ],
                    }
                    .encipher(),
                    pointer: Some(0),
                    refund: Some(0),
                    edicts: vec![],
                    from: None,
                    burn: None,
                    protocol_tag: protocol_id as u128,
                },
            ]
            .encipher()
            {
                Ok(v) => Some(v),
                Err(_) => None,
            },
        })
        .encipher();

        //     // op return is at output 1
        let op_return = TxOut {
            value: Amount::from_sat(0).to_sat(),
            script_pubkey: runestone,
        };

        test_block.txdata.push(Transaction {
            version: 1,
            lock_time: bitcoin::absolute::LockTime::ZERO,
            input: vec![txin],
            output: vec![txout, op_return],
        });
        assert!(Protorune::index_block::<AlkaneMessageContext>(
            test_block.clone(),
            block_height as u64
        )
        .is_ok());

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

    #[wasm_bindgen_test]
    async fn protomessage_with_binary_test() {
        clear();
        let block_height = 840000;
        let mut test_block = helpers::create_block_with_coinbase_tx(block_height);

        let wasm_binary = alkanes_std_test_build::get_bytes();
        let raw_envelope = RawEnvelope::from(wasm_binary);

        let witness = raw_envelope.to_witness();

        // Create a transaction input
        let input_cellpack = Cellpack {
            target: AlkaneId { block: 1, tx: 0 },
            inputs: vec![
                123456789123456789123456789u128,
                987654321987654321987654321u128,
            ],
        };

        test_block
            .txdata
            .push(alkane_helpers::create_cellpack_with_witness(
                witness,
                input_cellpack,
            ));
        assert!(Protorune::index_block::<AlkaneMessageContext>(
            test_block.clone(),
            block_height as u64
        )
        .is_ok());

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
}
