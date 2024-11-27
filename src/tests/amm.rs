use crate::tests::std::{alkanes_std_amm_pool_build, alkanes_std_auth_token_build};
use alkanes::message::AlkaneMessageContext;
use alkanes_support::cellpack::Cellpack;
use alkanes_support::constants::{AMM_FACTORY_ID, AUTH_TOKEN_FACTORY_ID};
use alkanes_support::id::AlkaneId;
use anyhow::Result;
use bitcoin::address::NetworkChecked;
use bitcoin::blockdata::transaction::OutPoint;
use bitcoin::hashes::Hash;
use bitcoin::{Address, Amount, ScriptBuf, Sequence, TxIn, TxOut, Witness};
use metashrew_support::index_pointer::KeyValuePointer;
use protobuf::{Message, MessageField};
use protorune::{
    balance_sheet::load_sheet, message::MessageContext, tables::RuneTable,
    view::protorunes_by_outpoint,
};
use protorune_support::balance_sheet::{BalanceSheet, ProtoruneRuneId};
use protorune_support::protostone::Protostone;
use protorune_support::protostone::ProtostoneEdict;
use protorune_support::utils::consensus_encode;

use crate::index_block;
use crate::tests::helpers::{self as alkane_helpers, assert_binary_deployed_to_id};
use crate::tests::std::{alkanes_std_amm_factory_build, alkanes_std_owned_token_build};
#[allow(unused_imports)]
use metashrew::{clear, get_cache, index_pointer::IndexPointer, println, stdio::stdout};
use std::fmt::Write;
use wasm_bindgen_test::wasm_bindgen_test;
#[wasm_bindgen_test]
fn test_amm_pool_normal() -> Result<()> {
    clear();
    let block_height = 840_000;
    let cellpacks: Vec<Cellpack> = [
        //amm pool factory init
        Cellpack {
            target: AlkaneId {
                block: 3,
                tx: AMM_FACTORY_ID,
            },
            inputs: vec![50],
        },
        //auth token factory init
        Cellpack {
            target: AlkaneId {
                block: 3,
                tx: AUTH_TOKEN_FACTORY_ID,
            },
            inputs: vec![100],
        },
        Cellpack {
            target: AlkaneId { block: 1, tx: 0 },
            inputs: vec![0],
        },
        // token 1 init 1 auth token and mint 1000000 owned tokens
        Cellpack {
            target: AlkaneId { block: 1, tx: 0 },
            inputs: vec![0, 1, 1000000],
        },
        // token 2 init 1 auth token and mint 1000000 owned tokens
        Cellpack {
            target: AlkaneId { block: 5, tx: 1 }, // factory creation of owned token using {2, 1} as the factory. Then it deploys to {2,3}
            inputs: vec![0, 1, 1000000],
        },
    ]
    .into();
    let amm_pool_factory = AlkaneId {
        block: 4,
        tx: AMM_FACTORY_ID,
    };
    let auth_token_factory = AlkaneId {
        block: 4,
        tx: AUTH_TOKEN_FACTORY_ID,
    };
    let amm_factory_deployment = AlkaneId { block: 2, tx: 0 };
    let owned_token_1_deployment = AlkaneId { block: 2, tx: 1 };
    let auth_token_1_deployment = AlkaneId { block: 2, tx: 2 };
    let owned_token_2_deployment = AlkaneId { block: 2, tx: 3 };
    let auth_token_2_deployment = AlkaneId { block: 2, tx: 4 };
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
                    id: owned_token_1_deployment.into(),
                    amount: 1000000,
                    output: 0,
                },
                ProtostoneEdict {
                    id: owned_token_2_deployment.into(),
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
                target: amm_factory_deployment,
                inputs: vec![1],
            }],
            OutPoint {
                txid: test_block.txdata[test_block.txdata.len() - 1].compute_txid(),
                vout: 0,
            },
            false,
        ),
    );
    let amm_pool_deployment = AlkaneId { block: 2, tx: 5 };
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
    let _ = assert_binary_deployed_to_id(
        amm_pool_factory.clone(),
        alkanes_std_amm_pool_build::get_bytes(),
    );
    let _ = assert_binary_deployed_to_id(
        auth_token_factory.clone(),
        alkanes_std_auth_token_build::get_bytes(),
    );

    let _ = assert_binary_deployed_to_id(
        amm_factory_deployment.clone(),
        alkanes_std_amm_factory_build::get_bytes(),
    );
    let _ = assert_binary_deployed_to_id(
        owned_token_1_deployment.clone(),
        alkanes_std_owned_token_build::get_bytes(),
    );
    let _ = assert_binary_deployed_to_id(
        owned_token_2_deployment.clone(),
        alkanes_std_owned_token_build::get_bytes(),
    );
    let _ = assert_binary_deployed_to_id(
        auth_token_1_deployment.clone(),
        alkanes_std_auth_token_build::get_bytes(),
    );
    let _ = assert_binary_deployed_to_id(
        auth_token_2_deployment.clone(),
        alkanes_std_auth_token_build::get_bytes(),
    );
    let _ = assert_binary_deployed_to_id(
        amm_pool_deployment.clone(),
        alkanes_std_amm_pool_build::get_bytes(),
    );
    println!("balances at end {:?}", sheet);
    assert_eq!(sheet.get(&amm_pool_deployment.into()), 999000);
    Ok(())
}

#[wasm_bindgen_test]
fn test_amm_pool_skewed() -> Result<()> {
    clear();
    let block_height = 840_000;
    let cellpacks: Vec<Cellpack> = [
        //amm pool factory init
        Cellpack {
            target: AlkaneId {
                block: 3,
                tx: AMM_FACTORY_ID,
            },
            inputs: vec![50],
        },
        // auth token init
        Cellpack {
            target: AlkaneId {
                block: 3,
                tx: AUTH_TOKEN_FACTORY_ID,
            },
            inputs: vec![100],
        },
        // amm factory init
        Cellpack {
            target: AlkaneId { block: 1, tx: 0 },
            inputs: vec![0],
        },
        // owned token 1 init
        Cellpack {
            target: AlkaneId { block: 1, tx: 0 },
            inputs: vec![0, 1, 1000000 / 2],
        },
        //owned token 2 init
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
    let split = alkane_helpers::create_protostone_tx_with_inputs_and_default_pointer(
        vec![TxIn {
            previous_output: OutPoint {
                txid: test_block.txdata.last().unwrap().compute_txid(),
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
        0,
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
    let _ptr = RuneTable::for_protocol(AlkaneMessageContext::protocol_tag())
        .OUTPOINT_TO_RUNES
        .select(&consensus_encode(&outpoint)?);
    let mut payload = protorune_support::proto::protorune::OutpointWithProtocol::new();
    payload.protocol = MessageField::some((1u128).into());
    payload.txid = outpoint.txid.as_byte_array().clone().to_vec();
    payload.vout = outpoint.vout;
    let response: BalanceSheet = protorunes_by_outpoint(
        &<Vec<u8> as AsRef<[u8]>>::as_ref(&payload.write_to_bytes().unwrap()).to_vec(),
    )
    .unwrap()
    .into();
    println!("{:?}", response);
    //    let sheet = load_sheet(&ptr);
    /*
    get_cache().iter().for_each(|(k, v)| {
      if v.len() < 300 { println!("{}: {}", format_key(&k.as_ref().clone()), hex::encode(&v.as_ref().clone())); }
    });
    */
    //   println!("balances at end {:?}", sheet);
    Ok(())
}
