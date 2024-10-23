use alkanes_runtime::runtime::AlkaneResponder;
use alkanes_support::{
    cellpack::Cellpack, context::Context, id::AlkaneId, parcel::AlkaneTransfer,
    response::CallResponse, witness::find_witness_payload,
};
use anyhow::{anyhow, Result};
use bitcoin::blockdata::transaction::Transaction;
use metashrew_support::{
    compat::{to_arraybuffer_layout, to_ptr},
    utils::consume_sized_int,
};
use hex;
use protorune_support::utils::consensus_decode;

#[derive(Default)]
struct AMMFactory(());

fn shift<T>(v: &mut Vec<T>) -> Option<T> {
    if v.is_empty() {
        None
    } else {
        Some(v.remove(0))
    }
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
                if self.load("/initialized".as_bytes().to_vec()).len() != 0 {
                    self.store("/initialized".as_bytes().to_vec(), vec![0x01]);
                    CallResponse::default()
                } else {
                    panic!("already initialized");
                }
            }
            1 => {
                if context.incoming_alkanes.0.len() != 2 {
                    panic!("must send two runes to initialize a pool");
                } else {
                    CallResponse::default()
                }
            }
            2 => {
                let mut response = CallResponse::default();
                response.alkanes = context.incoming_alkanes.clone();
                let mut cursor = std::io::Cursor::<Vec<u8>>::new(
                    self.load(
                        (String::from("/pools/")
                            + hex::encode(
                                context
                                    .incoming_alkanes.0.into_iter()
                                    .map(|v| <AlkaneId as Into<Vec<u8>>>::into(v.id))
                                    .flatten()
                                    .collect::<Vec<u8>>(),
                            )
                            .as_str())
                        .as_bytes()
                        .to_vec(),
                    ),
                );
                response.data = (&consume_sized_int::<u128>(&mut cursor).unwrap().to_le_bytes()).to_vec();
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
