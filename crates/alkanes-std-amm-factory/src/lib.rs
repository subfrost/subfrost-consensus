use alkanes_runtime::{runtime::AlkaneResponder, storage::StoragePointer};
use alkanes_support::{
    cellpack::Cellpack, context::Context, id::AlkaneId, parcel::AlkaneTransfer,
    response::CallResponse, witness::find_witness_payload,
};
use anyhow::{anyhow, Result};
use bitcoin::blockdata::transaction::Transaction;
use hex;
use metashrew_support::{
    index_pointer::{KeyValuePointer},
    compat::{to_arraybuffer_layout, to_ptr},
    utils::consume_sized_int,
};
use protorune_support::utils::consensus_decode;
use std::sync::{Arc};

#[derive(Default)]
struct AMMFactory(());

fn shift<T>(v: &mut Vec<T>) -> Option<T> {
    if v.is_empty() {
        None
    } else {
        Some(v.remove(0))
    }
}

pub fn take_two<T: Clone>(v: &Vec<T>) -> (T, T) {
  (v[0].clone(), v[1].clone())
}

pub fn sort_alkanes(a: AlkaneId, b: AlkaneId) -> (AlkaneId, AlkaneId) {
  if a < b {
    (a, b)
  } else {
    (b, a)
  }
}

pub fn join_ids(a: AlkaneId, b: AlkaneId) -> Vec<u8> {
  let mut result: Vec<u8> = a.into();
  result.extend(&b.into());
  result
}

pub fn join_ids_from_tuple(v: (AlkaneId, AlkaneId)) -> Vec<u8> {
  join_ids(v.0, v.1)
}

impl AMMFactory {
    pub fn pull_incoming(&self, context: &mut Context) -> Option<AlkaneTransfer> {
        let i = context
            .incoming_alkanes
            .0
            .iter()
            .position(|v| v.id == context.myself)?;
        Some(context.incoming_alkanes.0.remove(i))
    }
    pub fn only_owner(&self, v: Option<AlkaneTransfer>) -> Result<()> {
        if let Some(auth) = v {
            if auth.value < 1 {
                Err(anyhow!(
                    "must spend a balance of this alkane to the alkane to use as a proxy"
                ))
            } else {
                Ok(())
            }
        } else {
            Err(anyhow!(
                "must spend a balance of this alkane to the alkane to use as a proxy"
            ))
        }
    }
}

impl AlkaneResponder for AMMFactory {
    fn execute(&self) -> CallResponse {
        let mut context = self.context().unwrap();
        let mut inputs = context.inputs.clone();
        match shift(&mut inputs).unwrap() {
            0 => {
                let mut pointer = StoragePointer::from_keyword("/initialized");
                if pointer.get().len() == 0 {
                    pointer.set(Arc::new(vec![0x01]));
                    CallResponse::default()
                } else {
                    panic!("already initialized");
                }
            }
            1 => {
                if context.incoming_alkanes.0.len() != 2 {
                    panic!("must send two runes to initialize a pool");
                } else {
                    let (a, b) = sort_alkanes(take_two(&context.incoming_alkanes.0));
                    let next_sequence = self.sequence();
                    StoragePointer::from_keyword("/pools/").select(&a.clone().into()).select(&b.clone().into()).set(Arc::new(AlkaneId::new(2, next_sequence).into()));
                    self.call(&Cellpack { 
                      target: AlkaneId {
                        block: 6,
                        tx: 0xffef
                      },
                      inputs: vec![0]
                    }, &AlkaneTransferParcel(vec![ context.incoming_alkanes.0, context.incoming_alkanes.1 ]), self.fuel()).unwrap()
                }
            }
            2 => {
                let mut response = CallResponse::default();
                response.alkanes = context.incoming_alkanes.clone();
                let mut cursor = std::io::Cursor::<Vec<u8>>::new(
                    StoragePointer::from_keyword("/pools/")
                        .select(
                            &context
                                .incoming_alkanes
                                .0
                                .into_iter()
                                .map(|v| <AlkaneId as Into<Vec<u8>>>::into(v.id))
                                .flatten()
                                .collect::<Vec<u8>>(),
                        )
                        .get().as_ref().clone(),
                );
                response.data = (&consume_sized_int::<u128>(&mut cursor)
                    .unwrap()
                    .to_le_bytes())
                    .to_vec();
                response
            }
            _ => {
                panic!("unrecognized opcode");
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn __execute() -> i32 {
    let mut response = to_arraybuffer_layout(&AMMFactory::default().execute().serialize());
    to_ptr(&mut response) + 4
}
