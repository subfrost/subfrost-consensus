use anyhow::Result;
use bitcoin::{Transaction, Block};
use bitcoin::blockdata::block::{Header};
use bitcoin::pow::{CompactTarget};
use bitcoin::blockdata::block::{BlockHash, Version, TxMerkleNode};
use bitcoin::hashes::{Hash};
use crate::utils::{consume_sized_int, consume_varint, consume_exact, consensus_decode};

pub const VERSION_AUXPOW: u32 = 0x100;
pub const VERSION_POS_START: u32 = 0x200;
pub const VERSION_CHAIN_START: u32 = 0x10000;
pub const VERSION_CHAIN_ID: u32 = 20;

#[derive(Default, Clone, Debug)]
pub struct AuxpowVersion(u32);

impl AuxpowVersion {
  pub fn new(v: u32) -> Self {
    Self(v)
  }
  pub fn unwrap(&self) -> u32 {
    self.0
  }
  pub fn base_version(&self) -> u32 {
    self.unwrap() % VERSION_AUXPOW
  }
  pub fn chain_id(&self) -> u32 {
    self.unwrap() / VERSION_CHAIN_START
  }
  pub fn is_auxpow(&self) -> bool {
    self.unwrap() & VERSION_AUXPOW != 0
  }
  pub fn is_proof_of_stake(&self) -> bool {
    self.unwrap() & VERSION_POS_START != 0
  }
  pub fn is_legacy(&self) -> bool {
    self.unwrap() < VERSION_AUXPOW
  }
}

#[derive(Clone, Debug)]
pub struct Auxpow {
  pub chain_merkle_branch: Vec<TxMerkleNode>,
  pub chain_merkle_branch_side_mask: u32,
  pub parent_branch_header: AuxpowHeader
}

impl Auxpow {
  pub fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<Auxpow> {
    let mut chain_merkle_branch: Vec<TxMerkleNode> = vec![];
    for _ in 0..consume_varint(cursor)? {
      chain_merkle_branch.push(TxMerkleNode::from_byte_array(to_ref(&consume_exact(cursor, 0x20)?).try_into().unwrap()));
    }
    let chain_merkle_branch_side_mask = consume_sized_int::<u32>(cursor)?;
    let parent_branch_header = AuxpowHeader::parse_without_auxpow(cursor)?;
    Ok(Auxpow {
      chain_merkle_branch,
      chain_merkle_branch_side_mask,
      parent_branch_header
    })
  }
}

#[derive(Clone, Debug)]
pub struct AuxpowHeader {
  pub version: AuxpowVersion,
  pub prev_blockhash: BlockHash,
  pub merkle_root: TxMerkleNode,
  pub time: u32,
  pub bits: CompactTarget,
  pub nonce: u32,
  pub auxpow: Option<Box<Auxpow>>
}

impl Into<Version> for AuxpowVersion {
  fn into(self) -> Version {
    Version::from_consensus(self.0 as i32)
  }
}

impl Into<Header> for AuxpowHeader {
  fn into(self) -> Header {
    Header {
      version: self.version.into(),
      prev_blockhash: self.prev_blockhash,
      merkle_root: self.merkle_root,
      time: self.time,
      bits: self.bits,
      nonce: self.nonce
    }
  }
}

#[derive(Clone, Debug)]
pub struct AuxpowBlock {
  pub header: AuxpowHeader,
  pub txdata: Vec<Transaction>
}

impl AuxpowBlock {
  pub fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<AuxpowBlock> {
    let header = AuxpowHeader::parse(cursor)?;
    let mut txdata: Vec<Transaction> = vec![];
    for _ in 0..consume_varint(cursor)? {
      txdata.push(consensus_decode::<Transaction>(cursor)?);
    }
    Ok(AuxpowBlock {
      header,
      txdata
    })
  }
}

fn to_ref(v: &Vec<u8>) -> &[u8] {
  v.as_ref()
}

fn to_bytes32(v: &[u8]) -> [u8; 32] {
  v.try_into().unwrap()
}

impl AuxpowHeader {
  pub fn parse_without_auxpow(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<AuxpowHeader> {
    let version = AuxpowVersion(consume_sized_int::<u32>(cursor)?.into());
    let prev_blockhash: BlockHash = BlockHash::from_byte_array(to_ref(&consume_exact(cursor, 0x20)?).try_into().unwrap());
    let merkle_root: TxMerkleNode = TxMerkleNode::from_byte_array(to_ref(&consume_exact(cursor, 0x20)?).try_into().unwrap());
    let time: u32 = consume_sized_int::<u32>(cursor)?;
    let bits: CompactTarget = CompactTarget::from_consensus(consume_sized_int::<u32>(cursor)?);
    let nonce: u32 = consume_sized_int::<u32>(cursor)?;
    Ok(AuxpowHeader {
      version,
      prev_blockhash,
      merkle_root,
      time,
      bits,
      nonce,
      auxpow: None
    })
  }
  pub fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<AuxpowHeader> {
    let mut result = Self::parse_without_auxpow(cursor)?;
    result.auxpow = if result.version.is_auxpow() {
      Some(Box::new(Auxpow::parse(cursor)?))
    } else {
      None
    };
    Ok(result)
  }
}

