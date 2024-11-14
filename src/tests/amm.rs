use crate::tests::std::{alkanes_std_amm_pool_build, alkanes_std_auth_token_build};
use alkanes::message::AlkaneMessageContext;
use alkanes_support::cellpack::Cellpack;
use alkanes_support::id::AlkaneId;
use anyhow::Result;
use bitcoin::address::NetworkChecked;
use bitcoin::blockdata::transaction::OutPoint;
use bitcoin::{Address, Amount, ScriptBuf, Sequence, TxIn, TxOut, Witness};
use metashrew_support::index_pointer::KeyValuePointer;
use protorune::{balance_sheet::load_sheet, message::MessageContext, tables::RuneTable};
use protorune_support::balance_sheet::ProtoruneRuneId;
use protorune_support::protostone::Protostone;
use protorune_support::protostone::ProtostoneEdict;
use protorune_support::utils::consensus_encode;

use crate::index_block;
use crate::tests::helpers as alkane_helpers;
use crate::tests::std::{alkanes_std_amm_factory_build, alkanes_std_owned_token_build};
#[allow(unused_imports)]
use metashrew::{clear, get_cache, index_pointer::IndexPointer, println, stdio::stdout};
use std::fmt::Write;
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
                tx: 0xffef,
            },
            inputs: vec![50],
        },
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
            inputs: vec![0],
        },
        Cellpack {
            target: AlkaneId { block: 1, tx: 0 },
            inputs: vec![0, 1, 1000000],
        },
        Cellpack {
            target: AlkaneId { block: 5, tx: 1 },
            inputs: vec![0, 1, 1000000],
        },
    ]
    .into();
    let mut test_block = alkane_helpers::init_with_multiple_cellpacks_with_tx(
        [
            alkanes_std_amm_pool_build::get_bytes(),
            alkanes_std_auth_token_build::get_bytes(),
            alkanes_std_amm_factory_build::get_bytes(),
            alkanes_std_owned_token_build::get_bytes(),
            [].into(),
        ]
        .into(),
        cellpacks,
    );
    let address: Address<NetworkChecked> =
        protorune::test_helpers::get_address(&protorune::test_helpers::ADDRESS1);
    let script_pubkey = address.script_pubkey();
    let split = alkane_helpers::create_protostone_tx_with_inputs(
        vec![TxIn {
            previous_output: OutPoint {
                txid: test_block.txdata[test_block.txdata.len() - 1].compute_txid(),
                vout: 0,
            },
            script_sig: ScriptBuf::new(),
            sequence: Sequence::MAX,
            witness: Witness::new(),
        }],
        vec![
            TxOut {
                value: Amount::from_sat(546),
                script_pubkey: script_pubkey.clone(),
            },
            TxOut {
                value: Amount::from_sat(546),
                script_pubkey: script_pubkey.clone(),
            },
        ],
        Protostone {
            from: None,
            burn: None,
            protocol_tag: 1,
            message: vec![],
            pointer: Some(1),
            refund: None,
            edicts: vec![
                ProtostoneEdict {
                    id: ProtoruneRuneId { block: 2, tx: 1 },
                    amount: 1000000,
                    output: 0,
                },
                ProtostoneEdict {
                    id: ProtoruneRuneId { block: 2, tx: 3 },
                    amount: 1000000,
                    output: 0,
                },
            ],
        },
    );
    test_block.txdata.push(split);
    test_block.txdata.push(
        alkane_helpers::create_multiple_cellpack_with_witness_and_in(
            Witness::new(),
            vec![Cellpack {
                target: AlkaneId { block: 2, tx: 0 },
                inputs: vec![1],
            }],
            OutPoint {
                txid: test_block.txdata[test_block.txdata.len() - 1].compute_txid(),
                vout: 0,
            },
            false,
        ),
    );
    let len = test_block.txdata.len();
    let outpoint = OutPoint {
        txid: test_block.txdata[len - 1].compute_txid(),
        vout: 0,
    };

    index_block(&test_block, block_height)?;
    let ptr = RuneTable::for_protocol(AlkaneMessageContext::protocol_tag())
        .OUTPOINT_TO_RUNES
        .select(&consensus_encode(&outpoint)?);
    let sheet = load_sheet(&ptr);
    /*
    get_cache().iter().for_each(|(k, v)| {
      if v.len() < 300 { println!("{}: {}", format_key(&k.as_ref().clone()), hex::encode(&v.as_ref().clone())); }
    });
    */
    println!("balances at end {:?}", sheet);
    Ok(())
}

#[wasm_bindgen_test]
fn test_amm_pool_skewed() -> Result<()> {
    clear();
    let block_height = 840_000;
    let cellpacks: Vec<Cellpack> = [
        //auth token factory init
        Cellpack {
            target: AlkaneId {
                block: 3,
                tx: 0xffef,
            },
            inputs: vec![50],
        },
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
            inputs: vec![0],
        },
        Cellpack {
            target: AlkaneId { block: 1, tx: 0 },
            inputs: vec![0, 1, 1000000 / 2],
        },
        Cellpack {
            target: AlkaneId { block: 5, tx: 1 },
            inputs: vec![0, 1, 1000000],
        },
    ]
    .into();
    let mut test_block = alkane_helpers::init_with_multiple_cellpacks_with_tx(
        [
            alkanes_std_amm_pool_build::get_bytes(),
            alkanes_std_auth_token_build::get_bytes(),
            alkanes_std_amm_factory_build::get_bytes(),
            alkanes_std_owned_token_build::get_bytes(),
            [].into(),
        ]
        .into(),
        cellpacks,
    );
    let address: Address<NetworkChecked> =
        protorune::test_helpers::get_address(&protorune::test_helpers::ADDRESS1);
    let mut script_pubkey = address.script_pubkey();
    let split = alkane_helpers::create_protostone_tx_with_inputs(
        vec![TxIn {
            previous_output: OutPoint {
                txid: test_block.txdata[test_block.txdata.len() - 1].compute_txid(),
                vout: 0,
            },
            script_sig: ScriptBuf::new(),
            sequence: Sequence::MAX,
            witness: Witness::new(),
        }],
        vec![
            TxOut {
                value: Amount::from_sat(546),
                script_pubkey: script_pubkey.clone(),
            },
            TxOut {
                value: Amount::from_sat(546),
                script_pubkey: script_pubkey.clone(),
            },
        ],
        Protostone {
            from: None,
            burn: None,
            protocol_tag: 1,
            message: vec![],
            pointer: Some(1),
            refund: None,
            edicts: vec![
                ProtostoneEdict {
                    id: ProtoruneRuneId { block: 2, tx: 1 },
                    amount: 1000000 / 2,
                    output: 0,
                },
                ProtostoneEdict {
                    id: ProtoruneRuneId { block: 2, tx: 3 },
                    amount: 1000000,
                    output: 0,
                },
            ],
        },
    );
    test_block.txdata.push(split);
    test_block.txdata.push(
        alkane_helpers::create_multiple_cellpack_with_witness_and_in(
            Witness::new(),
            vec![Cellpack {
                target: AlkaneId { block: 2, tx: 0 },
                inputs: vec![1],
            }],
            OutPoint {
                txid: test_block.txdata[test_block.txdata.len() - 1].compute_txid(),
                vout: 0,
            },
            false,
        ),
    );
    let len = test_block.txdata.len();
    let outpoint = OutPoint {
        txid: test_block.txdata[len - 1].compute_txid(),
        vout: 0,
    };

    index_block(&test_block, block_height)?;
    let ptr = RuneTable::for_protocol(AlkaneMessageContext::protocol_tag())
        .OUTPOINT_TO_RUNES
        .select(&consensus_encode(&outpoint)?);
    let sheet = load_sheet(&ptr);
    /*
    get_cache().iter().for_each(|(k, v)| {
      if v.len() < 300 { println!("{}: {}", format_key(&k.as_ref().clone()), hex::encode(&v.as_ref().clone())); }
    });
    */
    println!("balances at end {:?}", sheet);
    Ok(())
}
