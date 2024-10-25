use alkanes_runtime::runtime::AlkaneResponder;
use alkanes_runtime::storage::StoragePointer;
use alkanes_support::{
    id::AlkaneId, parcel::AlkaneTransfer, response::CallResponse, utils::shift,
    witness::find_witness_payload,
};
use anyhow::{anyhow, Result};
use bitcoin::Transaction;
use metashrew_support::index_pointer::KeyValuePointer;
use metashrew_support::{
    compat::{to_arraybuffer_layout, to_ptr},
    utils::{consume_exact, consume_sized_int, consume_to_end},
};
use protorune_support::utils::consensus_decode;
use rs_merkle::{algorithms::Sha256, Hasher, MerkleProof};
use std::io::Cursor;
use std::sync::Arc;

#[derive(Default)]
struct MerkleDistributor(());

pub fn overflow_error(v: Option<u128>) -> Result<u128> {
    v.ok_or("").map_err(|_| anyhow!("overflow error"))
}

pub fn sub_fees(v: u128) -> Result<u128> {
    Ok(overflow_error(v.checked_mul(997))? / 1000)
}

impl MerkleDistributor {
    pub fn verify_output(&self, vout: u32) -> Result<u128> {
        let tx = consensus_decode::<Transaction>(&mut std::io::Cursor::new(self.transaction()))?;
        let mut cursor: Cursor<Vec<u8>> = Cursor::<Vec<u8>>::new(
            find_witness_payload(&tx, 0)
                .ok_or("")
                .map_err(|_| anyhow!("witness envelope at index 0 does not contain data"))?,
        );
        let leaf = consume_exact(&mut cursor, 40)?;
        let leaf_hash = Sha256::hash(&leaf);
        let proof = consume_to_end(&mut cursor)?;
        let mut leaf_cursor = Cursor::new(leaf.clone());
        let p2sh = consume_exact(&mut leaf_cursor, 20)?;
        let index = consume_sized_int::<u32>(&mut leaf_cursor)? as usize;
        let amount = consume_sized_int::<u128>(&mut leaf_cursor)?;
        if MerkleProof::<Sha256>::try_from(proof)?.verify(
            self.root()?,
            &[index],
            &[leaf_hash],
            self.length(),
        ) {
            if tx.output[vout as usize]
                .script_pubkey
                .clone()
                .into_bytes()
                .to_vec()
                != p2sh
            {
                Err(anyhow!("spendable output created does not match proof"))
            } else {
                Ok(amount)
            }
        } else {
            Err(anyhow!("proof verification failure"))
        }
    }
    pub fn length_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/length")
    }
    pub fn root_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/root")
    }
    pub fn set_length(&self, v: usize) {
        self.length_pointer().set_value::<usize>(v);
    }
    pub fn set_root(&self, v: Vec<u8>) {
        self.root_pointer().set(Arc::new(v))
    }
    pub fn length(&self) -> usize {
        self.length_pointer().get_value::<usize>()
    }
    pub fn root(&self) -> Result<[u8; 32]> {
        let root_vec: Vec<u8> = self.root_pointer().get().as_ref().clone();
        let root_bytes: &[u8] = root_vec.as_ref();
        root_bytes
            .try_into()
            .map_err(|_| anyhow!("root bytes in storage are not of length 32"))
    }
    pub fn alkane_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/alkane")
    }
    pub fn alkane(&self) -> Result<AlkaneId> {
        Ok(self.alkane_pointer().get().as_ref().clone().try_into()?)
    }
    pub fn set_alkane(&self, v: AlkaneId) {
        self.alkane_pointer().set(Arc::<Vec<u8>>::new(v.into()));
    }
}

pub fn shift_or_err(v: &mut Vec<u128>) -> Result<u128> {
    shift(v)
        .ok_or("")
        .map_err(|_| anyhow!("expected u128 value in list but list is exhausted"))
}

pub fn shift_id(v: &mut Vec<u128>) -> Result<AlkaneId> {
    let block = shift_or_err(v)?;
    let tx = shift_or_err(v)?;
    Ok(AlkaneId { block, tx })
}

pub fn shift_as_long(v: &mut Vec<u128>) -> Result<u64> {
    Ok(shift_or_err(v)?.try_into()?)
}

pub fn shift_root(v: &mut Vec<u128>) -> Result<Vec<u8>> {
    Ok((&[
        shift_as_long(v)?,
        shift_as_long(v)?,
        shift_as_long(v)?,
        shift_as_long(v)?,
    ])
        .to_vec()
        .into_iter()
        .rev()
        .fold(Vec::<u8>::new(), |mut r, v| {
            r.extend(&v.to_be_bytes());
            r
        }))
}

impl AlkaneResponder for MerkleDistributor {
    fn execute(&self) -> CallResponse {
        let context = self.context().unwrap();
        let mut inputs = context.inputs.clone();
        match shift(&mut inputs).unwrap() {
            0 => {
                let mut pointer = StoragePointer::from_keyword("/initialized");
                if pointer.get().len() == 0 {
                    pointer.set(Arc::new(vec![0x01]));
                    if context.incoming_alkanes.0.len() != 1 {
                        panic!("must send 1 alkane to lock for distribution");
                    }
                    self.set_alkane(context.incoming_alkanes.0[0].id.clone());
                    self.set_length(shift(&mut inputs).unwrap().try_into().unwrap());
                    self.set_root(shift_root(&mut inputs).unwrap());
                    CallResponse::default()
                } else {
                    panic!("already initialized");
                }
            }
            1 => {
                let mut response = CallResponse::forward(&context.incoming_alkanes);
                response.alkanes.0.push(AlkaneTransfer {
                    value: self.verify_output(context.pointer).unwrap(),
                    id: self.alkane().unwrap(),
                });
                response
            }
            _ => {
                panic!("opcode not recognized");
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn __execute() -> i32 {
    let mut response = to_arraybuffer_layout(&MerkleDistributor::default().execute().serialize());
    to_ptr(&mut response) + 4
}
