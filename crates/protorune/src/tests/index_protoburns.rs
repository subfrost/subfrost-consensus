#[cfg(test)]
mod tests {
    use crate::balance_sheet::load_sheet;
    use crate::message::{MessageContext, MessageContextParcel};
    use crate::test_helpers::{self as helpers};
    use crate::{tables, Protorune};
    use anyhow::Result;
    use bitcoin::OutPoint;
    use protorune_support::balance_sheet::{BalanceSheet, ProtoruneRuneId};
    use protorune_support::rune_transfer::RuneTransfer;
    use protorune_support::utils::consensus_encode;

    use metashrew::clear;
    use metashrew::{
        println,
        stdio::{stdout, Write},
    };
    use metashrew_support::index_pointer::KeyValuePointer;
    use std::str::FromStr;
    use wasm_bindgen_test::*;

    struct TestMessageContext(());

    impl MessageContext for TestMessageContext {
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

    /// In one runestone, etches a rune, then protoburns it
    #[wasm_bindgen_test]
    fn protoburn_test() {
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

        let protoburn_tx =
            helpers::create_default_protoburn_transaction(previous_output, protocol_id);

        test_block.txdata.push(protoburn_tx);
        assert!(Protorune::index_block::<TestMessageContext>(
            test_block.clone(),
            block_height as u64
        )
        .is_ok());

        // tx 0 is coinbase, tx 1 is runestone
        let outpoint_address: OutPoint = OutPoint {
            txid: test_block.txdata[1].compute_txid(),
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
            tx: 1,
        };
        // let v: Vec<u8> = protorune_id.into();
        let stored_balance_address = sheet.get(&protorune_id);
        assert_eq!(stored_balance_address, 0);
        let stored_protorune_balance = protorunes_sheet.get(&protorune_id);
        assert_eq!(stored_protorune_balance, 1000);
    }

    /// In one runestone, etches a rune, then protoburns it, then transfers it
    #[wasm_bindgen_test]
    fn protoburn_transfer_test() {
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

        let protoburn_tx =
            helpers::create_default_protoburn_transaction(previous_output, protocol_id);

        test_block.txdata.push(protoburn_tx.clone());

        let previous_output = OutPoint {
            txid: protoburn_tx.clone().compute_txid(),
            vout: 0,
        };
        let transfer_tx = helpers::create_protostone_transaction(
            previous_output,
            None,
            false,
            1,
            0,
            protocol_id,
            vec![],
        );

        test_block.txdata.push(transfer_tx);

        assert!(Protorune::index_block::<TestMessageContext>(
            test_block.clone(),
            block_height as u64
        )
        .is_ok());

        // tx 0 is coinbase, tx 1 is runestone, tx 2 is transfer
        let outpoint_address: OutPoint = OutPoint {
            txid: test_block.txdata[1].compute_txid(),
            vout: 0,
        };
        println!("output: {:?}", outpoint_address);
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
            tx: 1,
        };
        println!("protorune sheet for outpoint: {:?}", protorunes_sheet);
        // let v: Vec<u8> = protorune_id.into();
        let stored_balance_address = sheet.get(&protorune_id);
        assert_eq!(stored_balance_address, 0);
        let stored_protorune_balance = protorunes_sheet.get(&protorune_id);
        println!(
            "first tx stored_protorune_balance {}",
            stored_protorune_balance
        );
        assert_eq!(stored_protorune_balance, 0);

        let outpoint_address: OutPoint = OutPoint {
            txid: test_block.txdata[2].compute_txid(),
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
            tx: 1,
        };
        // let v: Vec<u8> = protorune_id.into();
        println!("balancesheet: {:?}", protorunes_sheet);
        let stored_balance_address = sheet.get(&protorune_id);
        assert_eq!(stored_balance_address, 0);
        let stored_protorune_balance = protorunes_sheet.get(&protorune_id);
        println!(
            "second tx stored_protorune_balance {}",
            stored_protorune_balance
        );
        assert_eq!(stored_protorune_balance, 1000);
    }

    /// In one runestone, etches a rune, then protoburns it, then transfers it
    #[wasm_bindgen_test]
    #[allow(non_snake_case)]
    fn protoburn_transfer_to_OP_RETURN() {
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

        let protoburn_tx =
            helpers::create_default_protoburn_transaction(previous_output, protocol_id);

        test_block.txdata.push(protoburn_tx.clone());

        let previous_output = OutPoint {
            txid: protoburn_tx.clone().compute_txid(),
            vout: 0,
        };

        let transfer_tx = helpers::create_protostone_transaction(
            previous_output,
            None,
            false,
            1,
            1,
            protocol_id,
            vec![],
        );

        test_block.txdata.push(transfer_tx);

        assert!(Protorune::index_block::<TestMessageContext>(
            test_block.clone(),
            block_height as u64
        )
        .is_ok());

        // tx 0 is coinbase, tx 1 is runestone, tx 2 is transfer
        let outpoint_address: OutPoint = OutPoint {
            txid: test_block.txdata[1].compute_txid(),
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
            tx: 1,
        };
        // let v: Vec<u8> = protorune_id.into();
        let stored_balance_address = sheet.get(&protorune_id);
        assert_eq!(stored_balance_address, 0);
        let stored_protorune_balance = protorunes_sheet.get(&protorune_id);
        println!(
            "first tx stored_protorune_balance {}",
            stored_protorune_balance
        );
        assert_eq!(stored_protorune_balance, 0);

        let outpoint_address: OutPoint = OutPoint {
            txid: test_block.txdata[2].compute_txid(),
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
            tx: 1,
        };
        // let v: Vec<u8> = protorune_id.into();
        let stored_balance_address = sheet.get(&protorune_id);
        assert_eq!(stored_balance_address, 0);
        let stored_protorune_balance = protorunes_sheet.get(&protorune_id);
        println!(
            "second tx stored_protorune_balance {}",
            stored_protorune_balance
        );
        assert_eq!(stored_protorune_balance, 0);
    }

    // TODO: Add more integration tests https://github.com/kungfuflex/alkanes-rs/issues/9
}
