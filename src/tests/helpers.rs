use alkanes_support::cellpack::Cellpack;
use alkanes_support::envelope::RawEnvelope;
use bitcoin::{
    address::NetworkChecked, Address, Amount, OutPoint, ScriptBuf, Sequence, TxIn, TxOut, Witness,
};
use bitcoin::{Block, Transaction};
use protorune::protostone::{Protostone, Protostones};
use protorune::test_helpers::{create_block_with_coinbase_tx, get_address, ADDRESS1};

use ordinals::{Etching, Rune, Runestone};
use std::str::FromStr;

use super::std::alkanes_std_test_build;

pub fn init_test_with_cellpack(cellpack: Cellpack) -> Block {
    let block_height = 840000;
    let mut test_block = create_block_with_coinbase_tx(block_height);

    let wasm_binary = alkanes_std_test_build::get_bytes();
    let raw_envelope = RawEnvelope::from(wasm_binary);

    let witness = raw_envelope.to_witness();

    // Create a transaction input

    test_block
        .txdata
        .push(create_cellpack_with_witness(witness, cellpack));
    test_block
}

pub fn create_cellpack_with_witness(witness: Witness, cellpack: Cellpack) -> Transaction {
    let previous_output = OutPoint {
        txid: bitcoin::Txid::from_str(
            "0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap(),
        vout: 0,
    };
    let protocol_id = 122;
    let input_script = ScriptBuf::new();
    let txin = TxIn {
        previous_output,
        script_sig: input_script,
        sequence: Sequence::MAX,
        witness,
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
                message: cellpack.encipher(),
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
    let address: Address<NetworkChecked> = get_address(&ADDRESS1);

    let script_pubkey = address.script_pubkey();
    let txout = TxOut {
        value: Amount::from_sat(100_000_000).to_sat(),
        script_pubkey,
    };
    Transaction {
        version: 1,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![txin],
        output: vec![txout, op_return],
    }
}
