use alkanes_support::cellpack::Cellpack;
use alkanes_support::envelope::RawEnvelope;
use alkanes_support::gz::{compress, decompress};
use alkanes_support::id::AlkaneId;
use anyhow::Result;
use bitcoin::blockdata::transaction::Version;
use bitcoin::{
    address::NetworkChecked, Address, Amount, OutPoint, ScriptBuf, Sequence, TxIn, TxOut, Witness,
};
use bitcoin::{Block, Transaction};
use metashrew::index_pointer::IndexPointer;
#[allow(unused_imports)]
use metashrew::{
    println,
    stdio::{stdout, Write},
};
use metashrew_support::index_pointer::KeyValuePointer;
use protorune::protostone::Protostones;
use protorune::test_helpers::{create_block_with_coinbase_tx, get_address, ADDRESS1};
use protorune_support::protostone::Protostone;

use ordinals::{Etching, Rune, Runestone};
use std::str::FromStr;

use super::std::alkanes_std_test_build;

pub fn init_test_with_cellpack(cellpack: Cellpack) -> Block {
    let block_height = 840000;
    let mut test_block = create_block_with_coinbase_tx(block_height);

    let wasm_binary = alkanes_std_test_build::get_bytes();
    let raw_envelope = RawEnvelope::from(wasm_binary);

    let witness = raw_envelope.to_gzipped_witness();

    // Create a transaction input

    test_block
        .txdata
        .push(create_cellpack_with_witness(witness, cellpack));
    test_block
}

pub fn init_with_multiple_cellpacks_with_tx(
    binaries: Vec<Vec<u8>>,
    cellpacks: Vec<Cellpack>,
) -> Block {
    let block_height = 840000;
    let mut test_block = create_block_with_coinbase_tx(block_height);
    let mut previous_out: Option<OutPoint> = None;
    let mut txs = binaries
        .into_iter()
        .zip(cellpacks.into_iter())
        .map(|i| {
            let (binary, cellpack) = i;
            let witness = if binary.len() == 0 {
                Witness::new()
            } else {
                RawEnvelope::from(binary).to_gzipped_witness()
            };
            if let Some(previous_output) = previous_out {
                let tx = create_multiple_cellpack_with_witness_and_in(
                    witness,
                    [cellpack].into(),
                    previous_output,
                    false,
                );
                previous_out = Some(OutPoint {
                    txid: tx.compute_txid(),
                    vout: 0,
                });
                tx
            } else {
                let tx = create_multiple_cellpack_with_witness(witness, [cellpack].into(), false);
                previous_out = Some(OutPoint {
                    txid: tx.compute_txid(),
                    vout: 0,
                });
                tx
            }
        })
        .collect::<Vec<Transaction>>();
    test_block.txdata.append(&mut txs);
    test_block
}

pub fn init_with_multiple_cellpacks(binary: Vec<u8>, cellpacks: Vec<Cellpack>) -> Block {
    let block_height = 840000;

    let mut test_block = create_block_with_coinbase_tx(block_height);

    let raw_envelope = RawEnvelope::from(binary);
    let witness = raw_envelope.to_gzipped_witness();
    test_block
        .txdata
        .push(create_multiple_cellpack_with_witness(
            witness, cellpacks, false,
        ));
    test_block
}

pub fn create_protostone_tx_with_inputs_and_default_pointer(
    inputs: Vec<TxIn>,
    outputs: Vec<TxOut>,
    protostone: Protostone,
    default_pointer: u32,
) -> Transaction {
    let runestone: ScriptBuf = (Runestone {
        etching: None,
        pointer: Some(default_pointer), // points to the OP_RETURN, so therefore targets the protoburn
        edicts: Vec::new(),
        mint: None,
        protocol: vec![protostone].encipher().ok(),
    })
    .encipher();
    let op_return = TxOut {
        value: Amount::from_sat(0),
        script_pubkey: runestone,
    };
    let address: Address<NetworkChecked> = get_address(&ADDRESS1);
    let _script_pubkey = address.script_pubkey();
    let mut _outputs = outputs.clone();
    _outputs.push(op_return);
    Transaction {
        version: Version::ONE,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: inputs,
        output: _outputs,
    }
}

pub fn create_protostone_tx_with_inputs(
    inputs: Vec<TxIn>,
    outputs: Vec<TxOut>,
    protostone: Protostone,
) -> Transaction {
    create_protostone_tx_with_inputs_and_default_pointer(inputs, outputs, protostone, 1)
}

pub fn create_multiple_cellpack_with_witness_and_in(
    witness: Witness,
    cellpacks: Vec<Cellpack>,
    previous_output: OutPoint,
    etch: bool,
) -> Transaction {
    let protocol_id = 1;
    let input_script = ScriptBuf::new();
    let txin = TxIn {
        previous_output,
        script_sig: input_script,
        sequence: Sequence::MAX,
        witness,
    };
    let protostones = [
        match etch {
            true => vec![Protostone {
                burn: Some(protocol_id),
                edicts: vec![],
                pointer: Some(4),
                refund: None,
                from: None,
                protocol_tag: 13, // this value must be 13 if protoburn
                message: vec![],
            }],
            false => vec![],
        },
        cellpacks
            .into_iter()
            .map(|cellpack| Protostone {
                message: cellpack.encipher(),
                pointer: Some(0),
                refund: Some(0),
                edicts: vec![],
                from: None,
                burn: None,
                protocol_tag: protocol_id as u128,
            })
            .collect(),
    ]
    .concat();
    let etching = if etch {
        Some(Etching {
            divisibility: Some(2),
            premine: Some(1000),
            rune: Some(Rune::from_str("TESTTESTTEST").unwrap()),
            spacers: Some(0),
            symbol: Some(char::from_str("A").unwrap()),
            turbo: true,
            terms: None,
        })
    } else {
        None
    };
    let runestone: ScriptBuf = (Runestone {
        etching,
        pointer: match etch {
            true => Some(1),
            false => Some(0),
        }, // points to the OP_RETURN, so therefore targets the protoburn
        edicts: Vec::new(),
        mint: None,
        protocol: protostones.encipher().ok(),
    })
    .encipher();

    //     // op return is at output 1
    let op_return = TxOut {
        value: Amount::from_sat(0),
        script_pubkey: runestone,
    };
    let address: Address<NetworkChecked> = get_address(&ADDRESS1);

    let script_pubkey = address.script_pubkey();
    let txout = TxOut {
        value: Amount::from_sat(100_000_000),
        script_pubkey,
    };
    Transaction {
        version: Version::ONE,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![txin],
        output: vec![txout, op_return],
    }
}

pub fn create_cellpack_with_witness(witness: Witness, cellpack: Cellpack) -> Transaction {
    create_multiple_cellpack_with_witness(witness, [cellpack].into(), false)
}

pub fn create_multiple_cellpack_with_witness(
    witness: Witness,
    cellpacks: Vec<Cellpack>,
    etch: bool,
) -> Transaction {
    let previous_output = OutPoint {
        txid: bitcoin::Txid::from_str(
            "0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap(),
        vout: 0,
    };
    create_multiple_cellpack_with_witness_and_in(witness, cellpacks, previous_output, etch)
}

pub fn assert_binary_deployed_to_id(token_id: AlkaneId, binary: Vec<u8>) -> Result<()> {
    let binary_1 = IndexPointer::from_keyword("/alkanes/")
        .select(&token_id.into())
        .get()
        .as_ref()
        .clone();
    let binary_2: Vec<u8> = compress(binary)?.into();
    assert_eq!(binary_1.len(), binary_2.len());
    assert_eq!(binary_1, binary_2);
    return Ok(());
}

pub fn assert_token_id_has_no_deployment(token_id: AlkaneId) -> Result<()> {
    let binary = IndexPointer::from_keyword("/alkanes/")
        .select(&token_id.into())
        .get()
        .as_ref()
        .clone();
    assert_eq!(binary.len(), 0);
    return Ok(());
}
