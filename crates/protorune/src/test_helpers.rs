use bitcoin::address::NetworkChecked;
use bitcoin::blockdata::block::{Block, Header};
use bitcoin::blockdata::script::ScriptBuf;
use bitcoin::blockdata::transaction::Version;
use bitcoin::blockdata::transaction::{Transaction, TxIn, TxOut};
use bitcoin::hashes::Hash;
use bitcoin::{Address, Amount, BlockHash, OutPoint, Sequence, Witness};
use byteorder::{ByteOrder, LittleEndian};
use core::str::FromStr;
use metashrew::{get_cache, println, stdio::stdout};
use metashrew_support::utils::format_key;
use ordinals::{Edict, Etching, Rune, RuneId, Runestone};
use protorune_support::balance_sheet::ProtoruneRuneId;
use std::fmt::Write;
use std::sync::Arc;

use crate::protostone::Protostones;
use protorune_support::protostone::{Protostone, ProtostoneEdict};

// TODO: This module should probably not be compiled into the prod indexer wasm
pub const ADDRESS1: &'static str = "bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu";

pub fn print_cache() {
    let cache = get_cache();

    for (key, value) in cache.iter() {
        let formatted_key = format_key(key);
        let formatted_value = format_key(value);

        println!("{}: {}", formatted_key, formatted_value);
    }
}
pub fn display_vec_as_hex(data: Vec<u8>) -> String {
    let mut hex_string = String::new();
    for byte in data {
        write!(&mut hex_string, "{:02x}", byte).expect("Unable to write");
    }
    hex_string
}

pub fn display_list_as_hex(data: Vec<Arc<Vec<u8>>>) -> String {
    let mut hex_string = String::new();

    for arc_data in data {
        for byte in arc_data.to_vec().iter() {
            write!(&mut hex_string, "{:02x}", byte).expect("Unable to write");
        }
    }

    hex_string
}

pub fn serialize_u32_little_endian(value: u32) -> Vec<u8> {
    let mut buf = vec![0u8; 4]; // Create a buffer of 4 bytes
    LittleEndian::write_u32(&mut buf, value); // Write the value in little-endian order
    buf
}

pub fn create_coinbase_transaction(height: u32) -> Transaction {
    // Create the script for the coinbase transaction
    let script_pubkey = Address::from_str("bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu")
        .unwrap()
        .require_network(bitcoin::Network::Bitcoin)
        .unwrap()
        .script_pubkey();
    // Create a coinbase transaction input
    let coinbase_input = TxIn {
        previous_output: Default::default(),
        script_sig: ScriptBuf::new(),
        sequence: Sequence::MAX, // sequence for coinbase
        witness: Witness::new(),
    };

    // Create the coinbase transaction output
    let coinbase_output = TxOut {
        value: Amount::from_sat(50_000_000), // 50 BTC in satoshis
        script_pubkey,
    };

    let locktime = bitcoin::absolute::LockTime::from_height(height).unwrap();

    // Create the coinbase transaction
    Transaction {
        version: Version::TWO,
        lock_time: locktime,
        input: vec![coinbase_input],
        output: vec![coinbase_output],
    }
}

pub fn serialize_block(block: &Block) -> [u8; 32] {
    block.block_hash().to_raw_hash().to_byte_array()
}

pub fn create_test_transaction() -> Transaction {
    create_test_transaction_with_witness(vec![])
}

pub fn create_test_transaction_with_witness(script: Vec<u8>) -> Transaction {
    let previous_output = OutPoint {
        txid: bitcoin::Txid::from_str(
            "0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap(),
        vout: 0,
    };
    let input_script = ScriptBuf::new();

    let mut witness = Witness::new();
    witness.push(&script);

    // Create a transaction input
    let txin = TxIn {
        previous_output,
        script_sig: input_script,
        sequence: Sequence::MAX,
        witness,
    };

    let address_str = "bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu";

    let address: Address<NetworkChecked> = Address::from_str(&address_str)
        .unwrap()
        .require_network(bitcoin::Network::Bitcoin)
        .unwrap();

    let script_pubkey = address.script_pubkey();

    let txout = TxOut {
        value: Amount::from_sat(100_000_000),
        script_pubkey,
    };

    Transaction {
        version: Version::ONE,
        lock_time: bitcoin::absolute::LockTime::ZERO, // no locktime
        input: vec![txin],
        output: vec![txout],
    }
}
#[derive(Debug, Clone)]
pub struct RunesTestingConfig {
    pub address1: String,
    pub address2: String,
    pub rune_name: String,
    pub rune_symbol: String,
    pub rune_etch_height: u64,
    pub rune_etch_vout: u32,
    // pub outpoints: Vec<OutPoint>,
}

impl RunesTestingConfig {
    pub fn new(
        address1: &str,
        address2: &str,
        rune_name: &str,
        rune_symbol: &str,
        rune_etch_height: u64,
        rune_etch_vout: u32,
    ) -> RunesTestingConfig {
        RunesTestingConfig {
            address1: String::from(address1),
            address2: String::from(address2),
            rune_name: String::from(rune_name),
            rune_symbol: String::from(rune_symbol),
            rune_etch_height,
            rune_etch_vout,
            // outpoints: vec![OutPoint {
            //     txid: bitcoin::Txid::from_str(
            //         "0000000000000000000000000000000000000000000000000000000000000000",
            //     )
            //     .unwrap(),
            //     vout: 0,
            // }],
        }
    }

    // pub fn get_previous_outpoint(&self) -> OutPoint {
    //     return self.outpoints.last().expect("not possible").clone();
    // }

    // pub fn add_outpoint(&mut self, outpoint: OutPoint) {
    //     self.outpoints.push(outpoint);
    // }
}

pub fn get_address(address: &str) -> Address<NetworkChecked> {
    Address::from_str(address)
        .unwrap()
        .require_network(bitcoin::Network::Bitcoin)
        .unwrap()
}

/// TODO: Convert all these create_*_transaction functions into sdk functions
/// Create a rune etching, transferring all runes to vout 0 in the tx
/// Mocks a dummy outpoint for the previous outpoint
pub fn create_rune_etching_transaction(config: &RunesTestingConfig) -> Transaction {
    let previous_output = OutPoint {
        txid: bitcoin::Txid::from_str(
            "0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap(),
        vout: 0,
    };
    let input_script = ScriptBuf::new();

    // Create a transaction input
    let txin = TxIn {
        previous_output,
        script_sig: input_script,
        sequence: Sequence::MAX,
        witness: Witness::new(),
    };

    let address: Address<NetworkChecked> = get_address(&config.address1);

    let script_pubkey = address.script_pubkey();

    // tx vout 0 will hold all 1000 of the runes
    let txout = TxOut {
        value: Amount::from_sat(100_000_000),
        script_pubkey,
    };

    let runestone: ScriptBuf = (Runestone {
        etching: Some(Etching {
            divisibility: Some(2),
            premine: Some(1000),
            rune: Some(Rune::from_str(&config.rune_name).unwrap()),
            spacers: Some(0),
            symbol: Some(char::from_str(&config.rune_symbol).unwrap()),
            turbo: true,
            terms: None,
        }),
        pointer: Some(0),
        edicts: Vec::new(),
        mint: None,
        protocol: None,
    })
    .encipher();

    let op_return = TxOut {
        value: Amount::from_sat(0),
        script_pubkey: runestone,
    };

    Transaction {
        version: Version::ONE,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![txin],
        output: vec![txout, op_return],
    }
}

///
/// TODO: Convert all these create_*_transaction functions into sdk functions
pub fn create_rune_transfer_transaction(
    config: &RunesTestingConfig,
    previous_output: OutPoint,
    edicts: Vec<Edict>,
) -> Transaction {
    let input_script = ScriptBuf::new();

    // Create a transaction input
    let txin = TxIn {
        previous_output,
        script_sig: input_script,
        sequence: Sequence::MAX,
        witness: Witness::new(),
    };

    let address1 = get_address(&config.address1);
    let address2 = get_address(&config.address2);

    let script_pubkey1 = address1.script_pubkey();
    let script_pubkey2 = address2.script_pubkey();

    // tx vout 0 corresponds to address2 will hold all 200 of the runes
    let txout0 = TxOut {
        value: Amount::from_sat(1),
        script_pubkey: script_pubkey2,
    };

    // tx vout 1 corresponds to address1 and will hold 800 of the runes
    let txout1 = TxOut {
        value: Amount::from_sat(99_999_999),
        script_pubkey: script_pubkey1,
    };

    let runestone: ScriptBuf = (Runestone {
        etching: None,
        pointer: Some(1), // refund to vout 1
        edicts,
        mint: None,
        protocol: None,
    })
    .encipher();

    let op_return = TxOut {
        value: Amount::from_sat(0),
        script_pubkey: runestone,
    };

    Transaction {
        version: Version::ONE,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![txin],
        output: vec![txout0, txout1, op_return],
    }
}

pub fn create_block_with_txs(txdata: Vec<Transaction>) -> Block {
    // Define block header fields
    let _version = Version::ONE;
    let previous_blockhash =
        BlockHash::from_str("00000000000000000005c3b409b4f17f9b3a97ed46d1a63d3f660d24168b2b3e")
            .unwrap();

    // let merkle_root_hash = bitcoin::merkle_tree::calculate_root(&[coinbase_tx.clone()]);
    let merkle_root = bitcoin::hash_types::TxMerkleNode::from_str(
        "4e07408562b4b5a9c0555f0671e0d2b6c5764c1d2a5e97c1d7f36f7c91e4c77a",
    )
    .unwrap();
    let time = 1231006505; // Example timestamp (January 3, 2009)
    let bits = bitcoin::CompactTarget::from_consensus(0x1234); // Example bits (difficulty)
    let nonce = 2083236893; // Example nonce

    // Create the block header
    let header = Header {
        version: bitcoin::blockdata::block::Version::from_consensus(1),
        prev_blockhash: previous_blockhash,
        merkle_root,
        time,
        bits,
        nonce,
    };

    // Create the block with the coinbase transaction
    Block { header, txdata }
}

pub fn create_block_with_sample_tx() -> Block {
    return create_block_with_txs(vec![create_test_transaction()]);
}

pub fn create_block_with_rune_tx() -> (Block, RunesTestingConfig) {
    let config = RunesTestingConfig::new(
        "bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu",
        "bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fym",
        "TESTER",
        "Z",
        840001,
        0,
    );
    return (
        create_block_with_txs(vec![create_rune_etching_transaction(&config)]),
        config,
    );
}

pub fn create_block_with_coinbase_tx(height: u32) -> Block {
    return create_block_with_txs(vec![create_coinbase_transaction(height)]);
}

/// Fixture with the following block:
///  - tx0:
///     - inputs:
///         - [0]: dummy outpoint
///     - outputs:
///         - [0]: ptpkh (?) address1
///         - [1]: runestone with etch 1000 runes to vout0
///  - tx1:
///     - inputs:
///         - [0]: outpoint(tx0, vout0)
///     - outputs:
///         - [0]: ptpkh address2
///         - [1]: ptpkh address1
///         - [2]: runestone with edict to transfer to vout0, default to vout1
pub fn create_block_with_rune_transfer(config: &RunesTestingConfig, edicts: Vec<Edict>) -> Block {
    let tx0 = create_rune_etching_transaction(config);
    let outpoint_with_runes = OutPoint {
        txid: tx0.compute_txid(),
        vout: 0,
    };

    let tx1 = create_rune_transfer_transaction(config, outpoint_with_runes, edicts);
    return create_block_with_txs(vec![tx0, tx1]);
}

pub fn create_protostone_encoded_tx(
    previous_output: OutPoint,
    protostones: Vec<Protostone>,
) -> Transaction {
    let input_script = ScriptBuf::new();

    // Create a transaction input
    let txin = TxIn {
        previous_output,
        script_sig: input_script,
        sequence: Sequence::MAX,
        witness: Witness::new(),
    };

    let address: Address<NetworkChecked> = get_address(&ADDRESS1);

    let script_pubkey = address.script_pubkey();

    let txout = TxOut {
        value: Amount::from_sat(100_000_000),
        script_pubkey,
    };

    let runestone: ScriptBuf = (Runestone {
        etching: None,
        pointer: None, // points to the OP_RETURN, so therefore targets the protoburn
        edicts: vec![Edict {
            id: RuneId {
                block: 840000,
                tx: 1,
            },
            amount: 500,
            output: 2,
        }],
        mint: None,
        protocol: match protostones.encipher() {
            Ok(v) => Some(v),
            Err(_) => None,
        },
    })
    .encipher();

    // op return is at output 1
    let op_return = TxOut {
        value: Amount::from_sat(0),
        script_pubkey: runestone,
    };

    Transaction {
        version: Version::ONE,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![txin],
        output: vec![txout, op_return],
    }
}

pub fn create_default_protoburn_transaction(
    previous_output: OutPoint,
    burn_protocol_id: u128,
) -> Transaction {
    // output rune pointer points to the OP_RETURN, so therefore targets the protoburn
    return create_protostone_transaction(
        previous_output,
        Some(burn_protocol_id),
        true,
        1,
        // protoburn and give protorunes to output 0
        0,
        13, // this value must be 13 if protoburn
        vec![],
    );
}

/// Create a protoburn given an input that holds runes
/// Outpoint with protorunes is the txid and vout 0
/// This outpoint holds 1000 protorunes
pub fn create_protostone_transaction(
    previous_output: OutPoint,
    burn_protocol_id: Option<u128>,
    etch: bool,
    output_rune_pointer: u32,
    output_protostone_pointer: u32,
    protocol_tag: u128,
    protostone_edicts: Vec<ProtostoneEdict>,
) -> Transaction {
    let input_script = ScriptBuf::new();

    // Create a transaction input
    let txin = TxIn {
        previous_output,
        script_sig: input_script,
        sequence: Sequence::MAX,
        witness: Witness::new(),
    };

    let address: Address<NetworkChecked> = get_address(&ADDRESS1);

    let script_pubkey = address.script_pubkey();

    let txout = TxOut {
        value: Amount::from_sat(100_000_000),
        script_pubkey,
    };

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
        pointer: Some(output_rune_pointer),
        edicts: Vec::new(),
        mint: None,
        protocol: match vec![Protostone {
            burn: burn_protocol_id,
            edicts: protostone_edicts,
            pointer: Some(output_protostone_pointer),
            refund: None,
            from: None,
            protocol_tag: protocol_tag,
            message: vec![],
        }]
        .encipher()
        {
            Ok(v) => Some(v),
            Err(_) => None,
        },
    })
    .encipher();

    // op return is at output 1
    let op_return = TxOut {
        value: Amount::from_sat(0),
        script_pubkey: runestone,
    };

    Transaction {
        version: Version::ONE,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![txin],
        output: vec![txout, op_return],
    }
}

pub fn create_protomessage_from_edict_tx(
    previous_output: OutPoint,
    protocol_id: u128,
    protorune_id: ProtoruneRuneId,
) -> Transaction {
    let input_script = ScriptBuf::new();
    let txin = TxIn {
        previous_output,
        script_sig: input_script,
        sequence: Sequence::MAX,
        witness: Witness::new(),
    };

    let address: Address<NetworkChecked> = get_address(&ADDRESS1);

    let txout0 = TxOut {
        value: Amount::from_sat(1),
        script_pubkey: address.script_pubkey(),
    };
    let txout1 = TxOut {
        value: Amount::from_sat(2),
        script_pubkey: address.script_pubkey(),
    };

    let runestone: ScriptBuf = (Runestone {
        etching: None,
        pointer: Some(2), // all leftover runes points to the OP_RETURN, so therefore targets the protoburn. in this case, there are no runes
        edicts: Vec::new(),
        mint: None,
        protocol: match vec![Protostone {
            // protomessage which should transfer protorunes to the pointer
            message: vec![1u8],
            pointer: Some(0),
            refund: Some(1),
            edicts: vec![ProtostoneEdict {
                id: protorune_id,
                amount: 800,
                output: 4, // output 0, 1 are the spendable outputs, output 2 is the op_return, output 3 is reserved, output 4 is the protomessage
            }],
            from: None,
            burn: None,
            protocol_tag: protocol_id as u128,
        }]
        .encipher()
        {
            Ok(v) => Some(v),
            Err(_) => None,
        },
    })
    .encipher();

    //     // op return is at output 1
    let op_return = TxOut {
        value: Amount::from_sat(0),
        script_pubkey: runestone,
    };

    Transaction {
        version: Version::ONE,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![txin],
        output: vec![txout0, txout1, op_return],
    }
}
