use alkanes_runtime::runtime::AlkaneResponder;
use alkanes_runtime::storage::StoragePointer;
use alkanes_support::{
    id::AlkaneId,
    parcel::{AlkaneTransfer, AlkaneTransferParcel},
    response::CallResponse,
};
use anyhow::{anyhow, Result};
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};
use metashrew_support::index_pointer::KeyValuePointer;
use num::integer::Roots;
use protorune_support::balance_sheet::BalanceSheet;
use std::sync::Arc;

#[derive(Default)]
struct AMMPool(());

fn shift<T>(v: &mut Vec<T>) -> Option<T> {
    if v.is_empty() {
        None
    } else {
        Some(v.remove(0))
    }
}

pub fn overflow_error(v: Option<u128>) -> Result<u128> {
    v.ok_or("").map_err(|_| anyhow!("overflow error"))
}

impl AMMPool {
    pub fn alkanes_for_self(&self) -> Result<(AlkaneId, AlkaneId)> {
        Ok((
            StoragePointer::from_keyword("/alkanes/0")
                .get()
                .as_ref()
                .clone()
                .try_into()?,
            StoragePointer::from_keyword("/alkanes/1")
                .get()
                .as_ref()
                .clone()
                .try_into()?,
        ))
    }
    pub fn check_inputs(&self, parcel: &AlkaneTransferParcel, n: usize) -> Result<()> {
        if parcel.0.len() > n {
            Err(anyhow!(format!(
                "{} alkanes sent but maximum {} supported",
                parcel.0.len(),
                n
            )))
        } else {
            let (a, b) = self.alkanes_for_self()?;
            if let Some(_) = parcel.0.iter().find(|v| v.id != a && v.id != b) {
                Err(anyhow!("unsupported alkane sent to pool"))
            } else {
                Ok(())
            }
        }
    }
    pub fn total_supply(&self) -> u128 {
        StoragePointer::from_keyword("/totalsupply").get_value::<u128>()
    }
    pub fn set_total_supply(&self, v: u128) {
        StoragePointer::from_keyword("/totalsupply").set_value::<u128>(v);
    }
    pub fn reserves(&self) -> (AlkaneTransfer, AlkaneTransfer) {
        let (a, b) = self.alkanes_for_self().unwrap();
        let context = self.context().unwrap();
        (
            AlkaneTransfer {
                id: a,
                value: self.balance(&context.myself, &a),
            },
            AlkaneTransfer {
                id: b,
                value: self.balance(&context.myself, &b),
            },
        )
    }
    pub fn previous_reserves(
        &self,
        parcel: &AlkaneTransferParcel,
    ) -> (AlkaneTransfer, AlkaneTransfer) {
        let (reserve_a, reserve_b) = self.reserves();
        let mut reserve_sheet: BalanceSheet =
            AlkaneTransferParcel(vec![reserve_a.clone(), reserve_b.clone()]).into();
        let incoming_sheet: BalanceSheet = parcel.clone().into();
        reserve_sheet.debit(&incoming_sheet).unwrap();
        (
            AlkaneTransfer {
                id: reserve_a.id.clone(),
                value: reserve_sheet.get(&reserve_a.id.clone().into()),
            },
            AlkaneTransfer {
                id: reserve_b.id.clone(),
                value: reserve_sheet.get(&reserve_b.id.clone().into()),
            },
        )
    }
    pub fn mint(&self, parcel: AlkaneTransferParcel) -> Result<CallResponse> {
        self.check_inputs(&parcel, 2)?;
        let total_supply = self.total_supply();
        let (reserve_a, reserve_b) = self.reserves();
        let (previous_a, previous_b) = self.previous_reserves(&parcel);
        let root_k_last = overflow_error(previous_a.value.checked_mul(previous_b.value))?.sqrt();
        let root_k = overflow_error(reserve_a.value.checked_mul(reserve_b.value))?.sqrt();
        if root_k > root_k_last {
            let numerator = overflow_error(
                total_supply.checked_mul(overflow_error(root_k.checked_sub(root_k_last))?),
            )?;
            let denominator =
                overflow_error(overflow_error(root_k.checked_mul(5))?.checked_add(root_k_last))?;
            let liquidity = numerator / denominator;
            self.set_total_supply(overflow_error(total_supply.checked_add(liquidity))?);
            let mut response = CallResponse::default();
            response.alkanes = AlkaneTransferParcel(vec![AlkaneTransfer {
                id: self.context().unwrap().myself,
                value: liquidity,
            }]);
            Ok(response)
        } else {
            Err(anyhow!("root k is less than previous root k"))
        }
    }
    pub fn burn(&self, parcel: &AlkaneTransferParcel) -> Result<CallResponse> {
        self.check_inputs(&parcel, 1)?;
        let incoming = parcel.0[0].clone();
        if incoming.id != self.context()?.myself {
            return Err(anyhow!("can only burn LP alkane for this pair"));
        }
        let liquidity = incoming.value;
        let (reserve_a, reserve_b) = self.reserves();
        let total_supply = self.total_supply();
        let mut response = CallResponse::default();
        let amount_a = overflow_error(liquidity.checked_mul(reserve_a.value))? / total_supply;
        let amount_b = overflow_error(liquidity.checked_mul(reserve_b.value))? / total_supply;
        if (amount_a == 0 || amount_b == 0) {
            return Err(anyhow!("insufficient liquidity!"));
        }
        self.set_total_supply(overflow_error(total_supply.checked_sub(liquidity))?);
        response.alkanes = AlkaneTransferParcel(vec![AlkaneTransfer {
          id: reserve_a.id,
          value: amount_a
        }, AlkaneTransfer {
          id: reserve_b.id,
          value: amount_b
        }]);
        Ok(CallResponse::default())
    }
    pub fn swap(&self) -> Result<CallResponse> {
        Ok(CallResponse::default())
    }
    pub fn pull_ids(&self, v: &mut Vec<u128>) -> Option<(AlkaneId, AlkaneId)> {
        let a_block = shift(v)?;
        let a_tx = shift(v)?;
        let b_block = shift(v)?;
        let b_tx = shift(v)?;
        Some((AlkaneId::new(a_block, a_tx), AlkaneId::new(b_block, b_tx)))
    }
}

impl AlkaneResponder for AMMPool {
    fn execute(&self) -> CallResponse {
        let context = self.context().unwrap();
        let mut inputs = context.inputs.clone();
        match shift(&mut inputs).unwrap() {
            0 => {
                let mut pointer = StoragePointer::from_keyword("/initialized");
                if pointer.get().len() == 0 {
                    pointer.set(Arc::new(vec![0x01]));
                    let (a, b) = self.pull_ids(&mut inputs).unwrap();
                    StoragePointer::from_keyword("/alkane/0").set(Arc::new(a.into()));
                    StoragePointer::from_keyword("/alkane/1").set(Arc::new(b.into()));
                    self.mint(context.incoming_alkanes).unwrap()
                } else {
                    panic!("already initialized");
                }
            }
            1 => self.mint(context.incoming_alkanes).unwrap(),
            2 => self.swap().unwrap(),
            _ => {
                panic!("unrecognized opcode");
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn __execute() -> i32 {
    let mut response = to_arraybuffer_layout(&AMMPool::default().execute().serialize());
    to_ptr(&mut response) + 4
}
