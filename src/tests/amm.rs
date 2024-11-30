use crate::tests::std::{alkanes_std_amm_pool_build, alkanes_std_auth_token_build};
use alkanes::message::AlkaneMessageContext;
use alkanes_support::cellpack::Cellpack;
use alkanes_support::constants::{AMM_FACTORY_ID, AUTH_TOKEN_FACTORY_ID};
use alkanes_support::id::AlkaneId;
use anyhow::Result;
use bitcoin::address::NetworkChecked;
use bitcoin::blockdata::transaction::OutPoint;
use bitcoin::{Address, Amount, Block, ScriptBuf, Sequence, TxIn, TxOut, Witness};
use metashrew_support::index_pointer::KeyValuePointer;
use num::integer::Roots;
use protorune::{balance_sheet::load_sheet, message::MessageContext, tables::RuneTable};
use protorune_support::balance_sheet::BalanceSheet;
use protorune_support::protostone::Protostone;
use protorune_support::protostone::ProtostoneEdict;
use protorune_support::utils::consensus_encode;

use crate::index_block;
use crate::tests::helpers::{
    self as alkane_helpers, assert_binary_deployed_to_id, assert_token_id_has_no_deployment,
};
use crate::tests::std::{alkanes_std_amm_factory_build, alkanes_std_owned_token_build};
#[allow(unused_imports)]
use metashrew::{clear, get_cache, index_pointer::IndexPointer, println, stdio::stdout};
use std::fmt::Write;
use wasm_bindgen_test::wasm_bindgen_test;

struct AmmTestDeploymentIds {
    amm_pool_factory: AlkaneId,
    auth_token_factory: AlkaneId,
    amm_factory_deployment: AlkaneId,
    owned_token_1_deployment: AlkaneId,
    auth_token_1_deployment: AlkaneId,
    owned_token_2_deployment: AlkaneId,
    auth_token_2_deployment: AlkaneId,
    amm_pool_deployment: AlkaneId,
}

// per uniswap docs, the first 1e3 wei of lp token minted are burned to mitigate attacks where the value of a lp token is raised too high easily
pub const MINIMUM_LIQUIDITY: u128 = 1000;

fn init_block_with_amm_pool() -> Result<(Block, AmmTestDeploymentIds)> {
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
    let test_block = alkane_helpers::init_with_multiple_cellpacks_with_tx(
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
    let deployed_ids = AmmTestDeploymentIds {
        amm_pool_factory: AlkaneId {
            block: 4,
            tx: AMM_FACTORY_ID,
        },
        auth_token_factory: AlkaneId {
            block: 4,
            tx: AUTH_TOKEN_FACTORY_ID,
        },
        amm_factory_deployment: AlkaneId { block: 2, tx: 0 },
        owned_token_1_deployment: AlkaneId { block: 2, tx: 1 },
        auth_token_1_deployment: AlkaneId { block: 2, tx: 2 },
        owned_token_2_deployment: AlkaneId { block: 2, tx: 3 },
        auth_token_2_deployment: AlkaneId { block: 2, tx: 4 },
        amm_pool_deployment: AlkaneId { block: 2, tx: 5 },
    };
    return Ok((test_block, deployed_ids));
}

fn assert_contracts_correct_ids(deployment_ids: &AmmTestDeploymentIds) -> Result<()> {
    let _ = assert_binary_deployed_to_id(
        deployment_ids.amm_pool_factory.clone(),
        alkanes_std_amm_pool_build::get_bytes(),
    );
    let _ = assert_binary_deployed_to_id(
        deployment_ids.auth_token_factory.clone(),
        alkanes_std_auth_token_build::get_bytes(),
    );

    let _ = assert_binary_deployed_to_id(
        deployment_ids.amm_factory_deployment.clone(),
        alkanes_std_amm_factory_build::get_bytes(),
    );
    let _ = assert_binary_deployed_to_id(
        deployment_ids.owned_token_1_deployment.clone(),
        alkanes_std_owned_token_build::get_bytes(),
    );
    let _ = assert_binary_deployed_to_id(
        deployment_ids.owned_token_2_deployment.clone(),
        alkanes_std_owned_token_build::get_bytes(),
    );
    let _ = assert_binary_deployed_to_id(
        deployment_ids.auth_token_1_deployment.clone(),
        alkanes_std_auth_token_build::get_bytes(),
    );
    let _ = assert_binary_deployed_to_id(
        deployment_ids.auth_token_2_deployment.clone(),
        alkanes_std_auth_token_build::get_bytes(),
    );
    let _ = assert_binary_deployed_to_id(
        deployment_ids.amm_pool_deployment.clone(),
        alkanes_std_amm_pool_build::get_bytes(),
    );
    Ok(())
}

fn insert_init_pool_liquidity_tx(
    amount1: u128,
    amount2: u128,
    test_block: &mut Block,
    deployment_ids: &AmmTestDeploymentIds,
) {
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
                    id: deployment_ids.owned_token_1_deployment.into(),
                    amount: amount1,
                    output: 0,
                },
                ProtostoneEdict {
                    id: deployment_ids.owned_token_2_deployment.into(),
                    amount: amount2,
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
                target: deployment_ids.amm_factory_deployment,
                inputs: vec![1],
            }],
            OutPoint {
                txid: test_block.txdata[test_block.txdata.len() - 1].compute_txid(),
                vout: 0,
            },
            false,
        ),
    );
}

fn insert_remove_liquidity_tx(
    amount: u128,
    test_block: &mut Block,
    deployment_ids: &AmmTestDeploymentIds,
) {
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
            edicts: vec![ProtostoneEdict {
                id: deployment_ids.amm_pool_deployment.into(),
                amount,
                output: 0,
            }],
        },
    );
    test_block.txdata.push(split);
    test_block.txdata.push(
        alkane_helpers::create_multiple_cellpack_with_witness_and_in(
            Witness::new(),
            vec![Cellpack {
                target: deployment_ids.amm_pool_deployment,
                inputs: vec![2],
            }],
            OutPoint {
                txid: test_block.txdata[test_block.txdata.len() - 1].compute_txid(),
                vout: 0,
            },
            false,
        ),
    );
}

fn calc_lp_balance_from_pool_init(amount1: u128, amount2: u128) -> u128 {
    if (amount1 * amount2).sqrt() < MINIMUM_LIQUIDITY {
        return 0;
    }
    return (amount1 * amount2).sqrt() - MINIMUM_LIQUIDITY;
}

fn get_sheet_with_lp(test_block: &Block) -> Result<(BalanceSheet)> {
    let len = test_block.txdata.len();
    let outpoint = OutPoint {
        txid: test_block.txdata[len - 1].compute_txid(),
        vout: 0,
    };
    let ptr = RuneTable::for_protocol(AlkaneMessageContext::protocol_tag())
        .OUTPOINT_TO_RUNES
        .select(&consensus_encode(&outpoint)?);
    let sheet = load_sheet(&ptr);
    println!("balances at lp outpoint {:?}", sheet);
    Ok((sheet))
}

fn check_init_liquidity_lp_balance(
    amount1: u128,
    amount2: u128,
    test_block: &Block,
    deployment_ids: &AmmTestDeploymentIds,
) -> Result<()> {
    let sheet = get_sheet_with_lp(test_block)?;
    println!(
        "expected amt {:?}",
        calc_lp_balance_from_pool_init(amount1, amount2)
    );
    assert_eq!(
        sheet.get(&deployment_ids.amm_pool_deployment.into()),
        calc_lp_balance_from_pool_init(amount1, amount2)
    );
    Ok(())
}

fn test_amm_pool_init_fixture(amount1: u128, amount2: u128) -> Result<()> {
    let block_height = 840_000;
    let (mut test_block, deployment_ids) = init_block_with_amm_pool()?;
    insert_init_pool_liquidity_tx(amount1, amount2, &mut test_block, &deployment_ids);
    index_block(&test_block, block_height)?;
    assert_contracts_correct_ids(&deployment_ids)?;
    check_init_liquidity_lp_balance(amount1, amount2, &test_block, &deployment_ids)?;
    Ok(())
}

#[wasm_bindgen_test]
fn test_amm_pool_normal_init() -> Result<()> {
    clear();
    test_amm_pool_init_fixture(1000000, 1000000)
}

#[wasm_bindgen_test]
fn test_amm_pool_skewed_init() -> Result<()> {
    clear();
    test_amm_pool_init_fixture(1000000 / 2, 1000000)
}

#[wasm_bindgen_test]
fn test_amm_pool_zero_init() -> Result<()> {
    clear();
    test_amm_pool_init_fixture(1000000, 1)
}

#[wasm_bindgen_test]
fn test_amm_pool_bad_init() -> Result<()> {
    clear();
    let block_height = 840_000;
    let (mut test_block, deployment_ids) = init_block_with_amm_pool()?;
    insert_init_pool_liquidity_tx(10000, 1, &mut test_block, &deployment_ids);
    index_block(&test_block, block_height)?;
    assert_token_id_has_no_deployment(deployment_ids.amm_pool_deployment);
    let sheet = get_sheet_with_lp(&test_block)?;
    assert_eq!(sheet.get(&deployment_ids.amm_pool_deployment.into()), 0);
    Ok(())
}

#[wasm_bindgen_test]
fn test_amm_pool_normal_burn() -> Result<()> {
    clear();
    let block_height = 840_000;
    let (amount1, amount2) = (1000000, 1000000);
    let total_lp = calc_lp_balance_from_pool_init(amount1, amount2);
    let (mut test_block, deployment_ids) = init_block_with_amm_pool()?;
    insert_init_pool_liquidity_tx(amount1, amount2, &mut test_block, &deployment_ids);
    insert_remove_liquidity_tx(total_lp, &mut test_block, &deployment_ids);
    index_block(&test_block, block_height)?;
    let sheet = get_sheet_with_lp(&test_block)?;
    assert_eq!(sheet.get(&deployment_ids.amm_pool_deployment.into()), 0);
    Ok(())
}
