#[cfg(test)]
mod tests {
    use crate::balance_sheet::load_sheet;
    use crate::message::{MessageContext, MessageContextParcel};
    use crate::protostone::Protostones;
    use crate::test_helpers::{self as helpers, get_address, ADDRESS1};
    use crate::{tables, Protorune};
    use anyhow::Result;
    use bitcoin::Transaction;
    use bitcoin::{
        address::NetworkChecked, Address, Amount, OutPoint, ScriptBuf, Sequence, TxIn, TxOut,
        Witness,
    };
    use protorune_support::balance_sheet::{BalanceSheet, ProtoruneRuneId};
    use protorune_support::protostone::{Protostone, ProtostoneEdict};
    use protorune_support::rune_transfer::RuneTransfer;
    use protorune_support::utils::consensus_encode;

    use metashrew::clear;
    use metashrew_support::index_pointer::KeyValuePointer;
    use ordinals::{Etching, Rune, Runestone};
    use std::str::FromStr;
    use wasm_bindgen_test::*;

    struct HalfForwardHalfRuntime(());
    struct HalfForwardHalfRefund(());
    struct FullRefund(());

    impl MessageContext for HalfForwardHalfRuntime {
        fn protocol_tag() -> u128 {
            122
        }
        // takes half of the first runes balance
        fn handle(parcel: &MessageContextParcel) -> Result<(Vec<RuneTransfer>, BalanceSheet)> {
            let mut new_runtime_balances = parcel.runtime_balances.clone();
            let mut runes = parcel.runes.clone();
            runes[0].value = runes[0].value / 2;
            let transfer = runes[0].clone();
            <BalanceSheet as TryFrom<Vec<RuneTransfer>>>::try_from(runes)?
                .pipe(&mut new_runtime_balances);
            // transfer protorunes to the pointer
            Ok((vec![transfer], *new_runtime_balances))
        }
    }
    impl MessageContext for HalfForwardHalfRefund {
        fn protocol_tag() -> u128 {
            122
        }
        fn handle(parcel: &MessageContextParcel) -> Result<(Vec<RuneTransfer>, BalanceSheet)> {
            let mut new_runtime_balances = parcel.runtime_balances.clone();
            let mut runes = parcel.runes.clone();
            runes[0].value = runes[0].value / 2;
            let transfer = runes[0].clone();
            // transfer protorunes to the pointer. the other half is unaccounted for, so it is refunded
            Ok((vec![transfer], *new_runtime_balances))
        }
    }
    impl MessageContext for FullRefund {
        fn protocol_tag() -> u128 {
            122
        }
        fn handle(parcel: &MessageContextParcel) -> Result<(Vec<RuneTransfer>, BalanceSheet)> {
            unimplemented!()
        }
    }

    fn protomessage_from_protoburn_fixture(protocol_id: u128) -> bitcoin::Block {
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
                    pointer: Some(4), // output 0 is the spendable outputs, output 1 is the op_return, output 2 is reserved, output 3 is the protoburn, so output 4 is the protomessage
                    refund: None,
                    from: None,
                    protocol_tag: 13, // this value must be 13 if protoburn
                    message: vec![],
                },
                Protostone {
                    // protomessage which should transfer protorunes to the pointer
                    message: vec![1u8],
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

        // op return is at output 1
        let op_return = TxOut {
            value: Amount::from_sat(0).to_sat(),
            script_pubkey: runestone,
        };

        helpers::create_block_with_txs(vec![Transaction {
            version: 1,
            lock_time: bitcoin::absolute::LockTime::ZERO,
            input: vec![txin],
            output: vec![txout, op_return],
        }])
    }

    fn protomessage_from_edict_fixture(protocol_id: u128, block_height: u128) -> bitcoin::Block {
        let first_mock_output = OutPoint {
            txid: bitcoin::Txid::from_str(
                "0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap(),
            vout: 0,
        };

        let protoburn_tx = helpers::create_protoburn_transaction(first_mock_output, protocol_id);
        let protorune_id = ProtoruneRuneId {
            block: block_height as u128,
            tx: 0,
        };

        // output 0 holds all the protorunes
        let protoburn_input = OutPoint {
            txid: protoburn_tx.txid(),
            vout: 0,
        };

        let protomessage_tx =
            helpers::create_protomessage_from_edict_tx(protoburn_input, protocol_id, protorune_id);

        helpers::create_block_with_txs(vec![protoburn_tx, protomessage_tx])
    }

    /// protomessage in the same transaction as a protoburn
    /// The protoburn will target the protomessage and directly transfer to it
    #[wasm_bindgen_test]
    fn protomessage_same_tx_as_protoburn_test() {
        clear();
        let block_height = 840000;
        let protocol_id = 122;

        let test_block = protomessage_from_protoburn_fixture(protocol_id);

        assert!(Protorune::index_block::<HalfForwardHalfRuntime>(
            test_block.clone(),
            block_height as u64
        )
        .is_ok());

        let outpoint_address: OutPoint = OutPoint {
            txid: test_block.txdata[0].txid(),
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

        let protorune_id = ProtoruneRuneId {
            block: block_height as u128,
            tx: 0,
        };
        let stored_runes_balance = sheet.get(&protorune_id);
        assert_eq!(stored_runes_balance, 0);

        let stored_protorune_balance = protorunes_sheet.get(&protorune_id);
        assert_eq!(stored_protorune_balance, 500);
    }

    /// protomessage from edict
    /// The first transaction is a protoburn. The next transaction is a protostone that
    /// has an edict that targets the protomessage
    #[wasm_bindgen_test]
    fn protomessage_from_edict_test() {
        clear();
        let block_height = 840000;
        let protocol_id = 122;

        let test_block = protomessage_from_edict_fixture(protocol_id, block_height);
        let protorune_id = ProtoruneRuneId {
            block: block_height as u128,
            tx: 0,
        };

        assert!(Protorune::index_block::<HalfForwardHalfRuntime>(
            test_block.clone(),
            block_height as u64
        )
        .is_ok());

        // tx 0 is protoburn, tx 1 is protomessage
        let outpoint_address0: OutPoint = OutPoint {
            txid: test_block.txdata[1].txid(),
            vout: 0,
        };
        let outpoint_address1: OutPoint = OutPoint {
            txid: test_block.txdata[1].txid(),
            vout: 1,
        };
        // check runes balance
        let sheet = load_sheet(
            &tables::RUNES
                .OUTPOINT_TO_RUNES
                .select(&consensus_encode(&outpoint_address0).unwrap()),
        );

        let protorunes_sheet0 = load_sheet(
            &tables::RuneTable::for_protocol(protocol_id.into())
                .OUTPOINT_TO_RUNES
                .select(&consensus_encode(&outpoint_address0).unwrap()),
        );
        let protorunes_sheet1 = load_sheet(
            &tables::RuneTable::for_protocol(protocol_id.into())
                .OUTPOINT_TO_RUNES
                .select(&consensus_encode(&outpoint_address1).unwrap()),
        );

        let stored_runes_balance = sheet.get(&protorune_id);
        assert_eq!(stored_runes_balance, 0);

        let stored_protorune_balance0 = protorunes_sheet0.get(&protorune_id);
        assert_eq!(stored_protorune_balance0, 400);
        let stored_protorune_balance1 = protorunes_sheet1.get(&protorune_id);
        assert_eq!(stored_protorune_balance1, 0);
    }

    /// Tests that a message context that does not account for some runes will refund it
    #[wasm_bindgen_test]
    fn protomessage_half_refund_test() {
        clear();
        let block_height = 840000;
        let protocol_id = 122;

        let test_block = protomessage_from_edict_fixture(protocol_id, block_height);
        let protorune_id = ProtoruneRuneId {
            block: block_height as u128,
            tx: 0,
        };

        assert!(
            Protorune::index_block::<FullRefund>(test_block.clone(), block_height as u64).is_ok()
        );

        // tx 0 is protoburn, tx 1 is protomessage
        let outpoint_address0: OutPoint = OutPoint {
            txid: test_block.txdata[1].txid(),
            vout: 0,
        };
        let outpoint_address1: OutPoint = OutPoint {
            txid: test_block.txdata[1].txid(),
            vout: 1,
        };
        // check runes balance
        let sheet = load_sheet(
            &tables::RUNES
                .OUTPOINT_TO_RUNES
                .select(&consensus_encode(&outpoint_address0).unwrap()),
        );

        let protorunes_sheet0 = load_sheet(
            &tables::RuneTable::for_protocol(protocol_id.into())
                .OUTPOINT_TO_RUNES
                .select(&consensus_encode(&outpoint_address0).unwrap()),
        );

        let protorunes_sheet1 = load_sheet(
            &tables::RuneTable::for_protocol(protocol_id.into())
                .OUTPOINT_TO_RUNES
                .select(&consensus_encode(&outpoint_address1).unwrap()),
        );

        let stored_runes_balance = sheet.get(&protorune_id);
        assert_eq!(stored_runes_balance, 0);

        let stored_protorune_balance0 = protorunes_sheet0.get(&protorune_id);
        assert_eq!(stored_protorune_balance0, 400);
        let stored_protorune_balance1 = protorunes_sheet1.get(&protorune_id);
        assert_eq!(stored_protorune_balance1, 400);
    }

    /// Tests that a message context that returns an invalid result will refund all
    #[wasm_bindgen_test]
    fn protomessage_full_refund_test() {
        clear();
        let block_height = 840000;
        let protocol_id = 122;

        let test_block = protomessage_from_edict_fixture(protocol_id, block_height);
        let protorune_id = ProtoruneRuneId {
            block: block_height as u128,
            tx: 0,
        };

        assert!(
            Protorune::index_block::<FullRefund>(test_block.clone(), block_height as u64).is_ok()
        );

        // tx 0 is protoburn, tx 1 is protomessage
        let outpoint_address0: OutPoint = OutPoint {
            txid: test_block.txdata[1].txid(),
            vout: 0,
        };
        let outpoint_address1: OutPoint = OutPoint {
            txid: test_block.txdata[1].txid(),
            vout: 1,
        };
        // check runes balance
        let sheet = load_sheet(
            &tables::RUNES
                .OUTPOINT_TO_RUNES
                .select(&consensus_encode(&outpoint_address0).unwrap()),
        );

        let protorunes_sheet0 = load_sheet(
            &tables::RuneTable::for_protocol(protocol_id.into())
                .OUTPOINT_TO_RUNES
                .select(&consensus_encode(&outpoint_address0).unwrap()),
        );

        let protorunes_sheet1 = load_sheet(
            &tables::RuneTable::for_protocol(protocol_id.into())
                .OUTPOINT_TO_RUNES
                .select(&consensus_encode(&outpoint_address1).unwrap()),
        );

        let stored_runes_balance = sheet.get(&protorune_id);
        assert_eq!(stored_runes_balance, 0);

        let stored_protorune_balance0 = protorunes_sheet0.get(&protorune_id);
        assert_eq!(stored_protorune_balance0, 0);
        let stored_protorune_balance1 = protorunes_sheet1.get(&protorune_id);
        assert_eq!(stored_protorune_balance1, 800);
    }
}
