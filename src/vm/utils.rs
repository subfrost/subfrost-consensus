use super::{AlkanesInstance, AlkanesRuntimeContext, AlkanesState};
use crate::utils::{pipe_storagemap_to, transfer_from};
use crate::vm::fuel::compute_extcall_fuel;
use alkanes_support::{
    cellpack::Cellpack, id::AlkaneId, parcel::AlkaneTransferParcel, response::ExtendedCallResponse,
    storage::StorageMap, utils::overflow_error, witness::find_witness_payload,
    gz::{decompress}
};
use anyhow::{anyhow, Result};
use metashrew::{
    index_pointer::{AtomicPointer, IndexPointer},
    println,
    stdio::{stdout, Write},
};
use metashrew_support::index_pointer::KeyValuePointer;

use std::sync::Arc;
use wasmi::*;

pub fn read_arraybuffer(data: &[u8], data_start: i32) -> Result<Vec<u8>> {
    if data_start < 4 {
        return Err(anyhow::anyhow!("memory error"));
    }
    let len =
        u32::from_le_bytes((data[((data_start - 4) as usize)..(data_start as usize)]).try_into()?);
    return Ok(Vec::<u8>::from(
        &data[(data_start as usize)..(((data_start as u32) + len) as usize)],
    ));
}

pub fn get_memory<'a>(caller: &mut Caller<'_, AlkanesState>) -> Result<Memory> {
    caller
        .get_export("memory")
        .ok_or(anyhow!("export was not memory region"))?
        .into_memory()
        .ok_or(anyhow!("export was not memory region"))
}

pub fn sequence_pointer(ptr: &AtomicPointer) -> AtomicPointer {
    ptr.derive(&IndexPointer::from_keyword("/alkanes/sequence"))
}

pub fn run_special_cellpacks(
    context: &mut AlkanesRuntimeContext,
    cellpack: &Cellpack,
) -> Result<(AlkaneId, AlkaneId, Arc<Vec<u8>>)> {
    let mut payload = cellpack.clone();
    let mut binary = Arc::<Vec<u8>>::new(vec![]);
    if cellpack.target.is_create() {
        let wasm_payload = Arc::new(
            find_witness_payload(&context.message.transaction, 0)
                .ok_or("finding witness payload failed for creation of alkane")
                .map_err(|_| anyhow!("used CREATE cellpack but no binary found in witness"))?,
        );
        let mut next_sequence_pointer = sequence_pointer(&context.message.atomic);
        let next_sequence = next_sequence_pointer.get_value::<u128>();
        payload.target = AlkaneId {
            block: 2,
            tx: next_sequence,
        };
        let mut pointer = context
            .message
            .atomic
            .keyword("/alkanes/")
            .select(&payload.target.clone().into());
        pointer.set(wasm_payload.clone());
        binary = Arc::new(decompress(wasm_payload.as_ref().clone())?);
        next_sequence_pointer.set_value(next_sequence + 1);
    } else if let Some(number) = cellpack.target.reserved() {
        let wasm_payload = Arc::new(
            find_witness_payload(&context.message.transaction, 0)
                .ok_or("finding witness payload failed for creation of alkane")
                .map_err(|_| {
                    anyhow!("used CREATERESERVED cellpack but no binary found in witness")
                })?,
        );
        payload.target = AlkaneId {
            block: 4,
            tx: number,
        };
        let mut ptr = context
            .message
            .atomic
            .keyword("/alkanes/")
            .select(&payload.target.clone().into());
        if ptr.get().as_ref().len() == 0 {
            ptr.set(wasm_payload.clone());
        } else {
            return Err(anyhow!(format!(
                "used CREATERESERVED cellpack but {} already holds a binary",
                number
            )));
        }
        binary = Arc::new(decompress(wasm_payload.clone().as_ref().clone())?);
    } else if let Some(factory) = cellpack.target.factory() {
        let mut next_sequence_pointer = sequence_pointer(&context.message.atomic);
        let next_sequence = next_sequence_pointer.get_value::<u128>();
        payload.target = AlkaneId::new(2, next_sequence);
        next_sequence_pointer.set_value(next_sequence + 1);
        let context_binary: Vec<u8> = context
            .message
            .atomic
            .keyword("/alkanes/")
            .select(&factory.clone().into())
            .get()
            .as_ref()
            .clone();
        let rc = Arc::new(context_binary);
        context
            .message
            .atomic
            .keyword("/alkanes/")
            .select(&payload.target.clone().into())
            .set(rc.clone());
        binary = Arc::new(decompress(rc.as_ref().clone())?);
    }
    Ok((
        context.myself.clone(),
        payload.target.clone(),
        binary.clone(),
    ))
}

#[derive(Clone, Default, Debug)]
pub struct SaveableExtendedCallResponse {
    pub result: ExtendedCallResponse,
    pub _from: AlkaneId,
    pub _to: AlkaneId,
}

impl From<ExtendedCallResponse> for SaveableExtendedCallResponse {
    fn from(v: ExtendedCallResponse) -> Self {
        let mut response = Self::default();
        response.result = v;
        response
    }
}

impl SaveableExtendedCallResponse {
    pub(super) fn associate(&mut self, context: &AlkanesRuntimeContext) {
        self._from = context.myself.clone();
        self._to = context.caller.clone();
    }
}

impl Saveable for SaveableExtendedCallResponse {
    fn from(&self) -> AlkaneId {
        self._from.clone()
    }
    fn to(&self) -> AlkaneId {
        self._to.clone()
    }
    fn storage_map(&self) -> StorageMap {
        self.result.storage.clone()
    }
    fn alkanes(&self) -> AlkaneTransferParcel {
        self.result.alkanes.clone()
    }
}

pub trait Saveable {
    fn from(&self) -> AlkaneId;
    fn to(&self) -> AlkaneId;
    fn storage_map(&self) -> StorageMap;
    fn alkanes(&self) -> AlkaneTransferParcel;
    fn save(&self, atomic: &mut AtomicPointer) -> Result<()> {
        pipe_storagemap_to(
            &self.storage_map(),
            &mut atomic
                .derive(&IndexPointer::from_keyword("/alkanes/").select(&self.from().into())),
        );
        transfer_from(
            &self.alkanes(),
            &mut atomic.derive(&IndexPointer::default()),
            &self.from().into(),
            &self.to().into(),
        )?;
        Ok(())
    }
}

pub fn run_after_special(
    context: AlkanesRuntimeContext,
    binary: Arc<Vec<u8>>,
    start_fuel: u64,
) -> Result<(ExtendedCallResponse, u64)> {
    let mut instance = AlkanesInstance::from_alkane(context, binary.clone(), start_fuel)?;
    let response = instance.execute()?;
    let storage_len = response.storage.serialize().len() as u64;
    let fuel_used = overflow_error(
        start_fuel
            .checked_sub(instance.store.get_fuel().unwrap())
            .and_then(|v: u64| -> Option<u64> {
                let computed_fuel = compute_extcall_fuel(storage_len).ok()?;
                let opt = v.checked_add(computed_fuel);
                opt
            }),
    )?;
    Ok((response, fuel_used))
}
pub fn prepare_context(
    context: &mut AlkanesRuntimeContext,
    caller: &AlkaneId,
    myself: &AlkaneId,
    delegate: bool,
) {
    if !delegate {
        context.caller = caller.clone();
        context.myself = myself.clone();
    }
}

pub fn run(
    mut context: AlkanesRuntimeContext,
    cellpack: &Cellpack,
    start_fuel: u64,
    delegate: bool,
) -> Result<(ExtendedCallResponse, u64)> {
    let (caller, myself, binary) = run_special_cellpacks(&mut context, cellpack)?;
    println!(
        "running special cellpack, caller: {:?}, myself: {:?}",
        caller, myself
    );
    prepare_context(&mut context, &caller, &myself, delegate);
    run_after_special(context, binary, start_fuel)
}

pub fn send_to_arraybuffer<'a>(
    caller: &mut Caller<'_, AlkanesState>,
    ptr: usize,
    v: &Vec<u8>,
) -> Result<i32> {
    let mem = get_memory(caller)?;
    mem.write(&mut *caller, ptr - 4, &v.len().to_le_bytes())
        .map_err(|_| anyhow!("failed to write ArrayBuffer"))?;
    mem.write(&mut *caller, ptr, v.as_slice())
        .map_err(|_| anyhow!("failed to write ArrayBuffer"))?;
    Ok(ptr.try_into()?)
}
