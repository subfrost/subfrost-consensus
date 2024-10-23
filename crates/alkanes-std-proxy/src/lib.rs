use alkanes_runtime::runtime::AlkaneResponder;
use alkanes_support::{
    cellpack::Cellpack, context::Context, parcel::AlkaneTransfer, response::CallResponse,
    witness::find_witness_payload,
};
use anyhow::{anyhow, Result};
use bitcoin::blockdata::transaction::Transaction;
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};
use protorune_support::utils::consensus_decode;

#[derive(Default)]
struct Proxy(());

fn shift<T>(v: &mut Vec<T>) -> Option<T> {
    if v.is_empty() {
        None
    } else {
        Some(v.remove(0))
    }
}

impl Proxy {
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

impl AlkaneResponder for Proxy {
    fn execute(&self) -> CallResponse {
        let mut context = self.context().unwrap();
        let mut inputs = context.inputs.clone();
        let auth = self.pull_incoming(&mut context);
        match shift(&mut inputs).unwrap() {
            0 => {
                if self.load("/initialized".as_bytes().to_vec()).len() != 0 {
                    let mut response: CallResponse = CallResponse::default();
                    response.alkanes = context.incoming_alkanes.clone();
                    response.alkanes.0.push(AlkaneTransfer {
                        id: context.myself.clone(),
                        value: 1,
                    });
                    self.store("/initialized".as_bytes().to_vec(), vec![0x01]);
                    response
                } else {
                    panic!("already initialized");
                }
            }
            1 => {
                self.only_owner(auth.clone()).unwrap();
                let witness_index = shift(&mut inputs).unwrap();
                let tx =
                    consensus_decode::<Transaction>(&mut std::io::Cursor::new(self.transaction()))
                        .unwrap();
                let cellpack = Cellpack::parse(&mut std::io::Cursor::new(
                    find_witness_payload(&tx, witness_index.try_into().unwrap()).unwrap(),
                ))
                .unwrap();
                let mut response: CallResponse = self
                    .call(&cellpack, &context.incoming_alkanes, self.fuel())
                    .unwrap();
                response.alkanes.0.push(auth.unwrap());
                response
            }
            2 => {
                self.only_owner(auth.clone()).unwrap();
                let witness_index = shift(&mut inputs).unwrap();
                let tx =
                    consensus_decode::<Transaction>(&mut std::io::Cursor::new(self.transaction()))
                        .unwrap();
                let cellpack = Cellpack::parse(&mut std::io::Cursor::new(
                    find_witness_payload(&tx, witness_index.try_into().unwrap()).unwrap(),
                ))
                .unwrap();
                let mut response: CallResponse = self
                    .delegatecall(&cellpack, &context.incoming_alkanes, self.fuel())
                    .unwrap();
                response.alkanes.0.push(auth.unwrap());
                response
            }
            3 => {
                self.only_owner(auth.clone()).unwrap();
                let cellpack: Cellpack = inputs.try_into().unwrap();
                let mut response: CallResponse = self
                    .call(&cellpack, &context.incoming_alkanes, self.fuel())
                    .unwrap();
                response.alkanes.0.push(auth.unwrap());
                response
            }
            4 => {
                self.only_owner(auth.clone()).unwrap();
                let cellpack: Cellpack = inputs.try_into().unwrap();
                let mut response: CallResponse = self
                    .delegatecall(&cellpack, &context.incoming_alkanes, self.fuel())
                    .unwrap();
                response.alkanes.0.push(auth.unwrap());
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
    let mut response = to_arraybuffer_layout(&Proxy::default().execute().serialize());
    to_ptr(&mut response) + 4
}
