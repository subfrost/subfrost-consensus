#[cfg(test)]
mod tests {
    use alkanes_support::cellpack::Cellpack;
    use alkanes_support::id::AlkaneId;
    use protorune::test_helpers::{self as helpers};
    use protorune::Protorune;

    use crate::tests::helpers as alkane_helpers;
    use metashrew::clear;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::message::AlkaneMessageContext;
    use alkanes_support::envelope::RawEnvelope;

    use crate::tests::std::alkanes_std_test_build;

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
