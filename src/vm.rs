use crate::utils::{pipe_storagemap_to, transfer_from};
use alkanes_support::{
    cellpack::Cellpack, id::AlkaneId, parcel::AlkaneTransferParcel, response::ExtendedCallResponse,
    storage::StorageMap, witness::find_witness_payload,
};
use anyhow::{anyhow, Result};
use metashrew::index_pointer::{AtomicPointer, IndexPointer};
use metashrew::{println, stdio::stdout};
use metashrew_support::index_pointer::KeyValuePointer;

use protorune::message::MessageContextParcel;
use protorune_support::utils::consensus_encode;
use std::fmt::Write;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use wasmi::*;

#[derive(Default, Clone)]
pub struct AlkanesRuntimeContext {
    pub myself: AlkaneId,
    pub caller: AlkaneId,
    pub incoming_alkanes: AlkaneTransferParcel,
    pub returndata: Vec<u8>,
    pub inputs: Vec<u128>,
    pub message: Box<MessageContextParcel>,
}

impl AlkanesRuntimeContext {
    pub fn from_parcel_and_cellpack(
        message: &MessageContextParcel,
        cellpack: &Cellpack,
    ) -> AlkanesRuntimeContext {
        let cloned = cellpack.clone();
        let message_copy = message.clone();
        let incoming_alkanes = message_copy.runes.clone().into();
        AlkanesRuntimeContext {
            message: Box::new(message_copy),
            returndata: vec![],
            incoming_alkanes,
            myself: AlkaneId::default(),
            caller: AlkaneId::default(),
            inputs: cloned.inputs,
        }
    }
    pub fn flatten(&self) -> Vec<u128> {
        let mut result = Vec::<u128>::new();
        result.push(self.myself.block);
        result.push(self.myself.tx);
        result.push(self.caller.block);
        result.push(self.caller.tx);
        result.push(self.message.vout as u128);
        result.push(self.incoming_alkanes.0.len() as u128);
        for incoming in &self.incoming_alkanes.0 {
            result.push(incoming.id.block);
            result.push(incoming.id.tx);
            result.push(incoming.value);
        }
        for input in self.inputs.clone() {
            result.push(input);
        }
        result
    }
    pub fn serialize(&self) -> Vec<u8> {
        let result = self
            .flatten()
            .into_iter()
            .map(|v| {
                let ar = (&v.to_le_bytes()).to_vec();
                ar
            })
            .flatten()
            .collect::<Vec<u8>>();
        result
    }
}

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

pub struct AlkanesState {
    pub had_failure: bool,
    pub context: Arc<Mutex<AlkanesRuntimeContext>>,
    pub limiter: StoreLimits,
}

pub struct AlkanesInstance {
    instance: Instance,
    store: Store<AlkanesState>,
}

pub fn get_memory<'a>(caller: &mut Caller<'_, AlkanesState>) -> Result<Memory> {
    caller
        .get_export("memory")
        .ok_or(anyhow!("export was not memory region"))?
        .into_memory()
        .ok_or(anyhow!("export was not memory region"))
}

const MEMORY_LIMIT: usize = 33554432;

pub trait Extcall {
    fn isdelegate() -> bool;
    fn isstatic() -> bool;
    fn handle_atomic(atomic: &mut AtomicPointer) {
        if Self::isstatic() {
            atomic.rollback();
        } else {
            atomic.commit();
        }
    }
    fn change_context(
        target: AlkaneId,
        caller: AlkaneId,
        myself: AlkaneId,
    ) -> (AlkaneId, AlkaneId) {
        if Self::isdelegate() {
            (caller, myself)
        } else {
            (myself, target)
        }
    }
}

pub struct Call(());

impl Extcall for Call {
    fn isdelegate() -> bool {
        false
    }
    fn isstatic() -> bool {
        false
    }
}

pub struct Delegatecall(());

impl Extcall for Delegatecall {
    fn isdelegate() -> bool {
        true
    }
    fn isstatic() -> bool {
        false
    }
}

pub struct Staticcall(());

impl Extcall for Staticcall {
    fn isdelegate() -> bool {
        false
    }
    fn isstatic() -> bool {
        true
    }
}

pub struct AlkanesHostFunctionsImpl(());
impl AlkanesHostFunctionsImpl {
    fn _abort<'a>(caller: Caller<'_, AlkanesState>) {
        AlkanesHostFunctionsImpl::abort(caller, 0, 0, 0, 0);
    }
    fn abort<'a>(mut caller: Caller<'_, AlkanesState>, _: i32, _: i32, _: i32, _: i32) {
        caller.data_mut().had_failure = true;
    }
    fn request_storage<'a>(caller: &mut Caller<'_, AlkanesState>, k: i32) -> Result<i32> {
        let mem = get_memory(caller)?;
        let key = {
            let data = mem.data(&caller);
            read_arraybuffer(data, k)?
        };
        let myself = caller.data_mut().context.lock().unwrap().myself.clone();
        Ok(caller
            .data_mut()
            .context
            .lock()
            .unwrap()
            .message
            .atomic
            .keyword("/alkanes/")
            .select(&myself.into())
            .keyword("/storage")
            .select(&key)
            .get()
            .len()
            .try_into()?)
    }
    fn load_storage<'a>(caller: &mut Caller<'_, AlkanesState>, k: i32, v: i32) -> Result<i32> {
        let mem = get_memory(caller)?;
        let key = {
            let data = mem.data(&caller);
            read_arraybuffer(data, k)?
        };
        let value = {
            let myself = caller.data_mut().context.lock().unwrap().myself.clone();
            (&caller.data_mut().context.lock().unwrap().message)
                .atomic
                .keyword("/alkanes/")
                .select(&myself.into())
                .keyword("/storage")
                .select(&key)
                .get()
        };
        send_to_arraybuffer(caller, v.try_into()?, value.as_ref())
    }
    fn request_context(caller: &mut Caller<'_, AlkanesState>) -> Result<i32> {
        Ok(caller
            .data_mut()
            .context
            .lock()
            .unwrap()
            .serialize()
            .len()
            .try_into()?)
    }
    fn load_context(caller: &mut Caller<'_, AlkanesState>, v: i32) -> Result<i32> {
        let context = caller.data_mut().context.lock().unwrap().serialize();
        send_to_arraybuffer(caller, v.try_into()?, &context)
    }
    fn request_transaction(caller: &mut Caller<'_, AlkanesState>) -> Result<i32> {
        Ok(consensus_encode(
            &caller
                .data_mut()
                .context
                .lock()
                .unwrap()
                .message
                .transaction,
        )?
        .len()
        .try_into()?)
    }
    fn returndatacopy(caller: &mut Caller<'_, AlkanesState>, output: i32) -> Result<()> {
        let context = caller.data_mut().context.lock().unwrap().returndata.clone();
        send_to_arraybuffer(caller, output.try_into()?, &context)?;
        Ok(())
    }
    fn load_transaction(caller: &mut Caller<'_, AlkanesState>, v: i32) -> Result<()> {
        let context = consensus_encode(
            &caller
                .data_mut()
                .context
                .lock()
                .unwrap()
                .message
                .transaction,
        )?;
        send_to_arraybuffer(caller, v.try_into()?, &context)?;
        Ok(())
    }
    fn request_block(caller: &mut Caller<'_, AlkanesState>) -> Result<i32> {
        Ok(
            consensus_encode(&caller.data_mut().context.lock().unwrap().message.block)?
                .len()
                .try_into()?,
        )
    }
    fn load_block(caller: &mut Caller<'_, AlkanesState>, v: i32) -> Result<()> {
        let context = consensus_encode(&caller.data_mut().context.lock().unwrap().message.block)?;
        send_to_arraybuffer(caller, v.try_into()?, &context)?;
        Ok(())
    }
    fn sequence(caller: &mut Caller<'_, AlkanesState>, output: i32) -> Result<()> {
        let buffer: Vec<u8> =
            (&sequence_pointer(&caller.data_mut().context.lock().unwrap().message.atomic)
                .get_value::<u128>()
                .to_le_bytes())
                .to_vec();
        send_to_arraybuffer(caller, output.try_into()?, &buffer)?;
        Ok(())
    }
    fn fuel(caller: &mut Caller<'_, AlkanesState>, output: i32) -> Result<()> {
        let buffer: Vec<u8> = (&caller.get_fuel().unwrap().to_le_bytes()).to_vec();
        send_to_arraybuffer(caller, output.try_into()?, &buffer)?;
        Ok(())
    }
    fn balance<'a>(
        caller: &mut Caller<'a, AlkanesState>,
        who_ptr: i32,
        what_ptr: i32,
        output: i32,
    ) -> Result<()> {
        let (who, what) = {
            let mem = get_memory(caller)?;
            let data = mem.data(&caller);
            (
                AlkaneId::parse(&mut Cursor::new(read_arraybuffer(data, who_ptr)?))?,
                AlkaneId::parse(&mut Cursor::new(read_arraybuffer(data, what_ptr)?))?,
            )
        };
        let balance = caller
            .data_mut()
            .context
            .lock()
            .unwrap()
            .message
            .atomic
            .keyword("/alkanes/")
            .select(&who.into())
            .keyword("/balances/")
            .select(&what.into())
            .get()
            .as_ref()
            .clone();
        send_to_arraybuffer(caller, output.try_into()?, &balance)?;
        Ok(())
    }
    fn extcall<'a, T: Extcall>(
        caller: &mut Caller<'_, AlkanesState>,
        cellpack_ptr: i32,
        incoming_alkanes_ptr: i32,
        checkpoint_ptr: i32,
        start_fuel: u64,
    ) -> Result<i32> {
        let mem = get_memory(caller)?;
        let data = mem.data(&caller);
        let buffer = read_arraybuffer(data, cellpack_ptr)?;
        let cellpack = Cellpack::parse(&mut Cursor::new(buffer))?;
        let buf = read_arraybuffer(data, incoming_alkanes_ptr)?;
        let incoming_alkanes = AlkaneTransferParcel::parse(&mut Cursor::new(buf))?;
        let storage_map =
            StorageMap::parse(&mut Cursor::new(read_arraybuffer(data, checkpoint_ptr)?))?;
        let subcontext = {
            let mut context = caller.data_mut().context.lock().unwrap();
            context.message.atomic.checkpoint();
            pipe_storagemap_to(
                &storage_map,
                &mut context.message.atomic.derive(
                    &IndexPointer::from_keyword("/alkanes/").select(&context.myself.into()),
                ),
            );
            if let Err(_) = transfer_from(
                &incoming_alkanes,
                &mut context.message.atomic.derive(&IndexPointer::default()),
                &context.myself,
                &cellpack.target,
            ) {
                context.message.atomic.rollback();
                context.returndata = Vec::<u8>::new();
                return Ok(0);
            }
            let mut subbed = (&*context).clone();
            subbed.message.atomic = context.message.atomic.derive(&IndexPointer::default());
            (subbed.caller, subbed.myself) = T::change_context(
                cellpack.target.clone(),
                context.caller.clone(),
                context.myself.clone(),
            );
            subbed.returndata = vec![];
            subbed.incoming_alkanes = incoming_alkanes.clone();
            subbed.inputs = cellpack.inputs.clone();
            subbed
        };
        match run(subcontext, &cellpack, start_fuel, T::isdelegate()) {
            Ok(response) => {
                let mut context = caller.data_mut().context.lock().unwrap();
                T::handle_atomic(&mut context.message.atomic);
                let serialized = response.serialize();
                context.returndata = serialized;
                Ok(context.returndata.len().try_into()?)
            }
            Err(_) => {
                let mut context = caller.data_mut().context.lock().unwrap();
                context.message.atomic.rollback();
                context.returndata = vec![];
                Ok(0)
            }
        }
    }
    fn log<'a>(caller: &mut Caller<'_, AlkanesState>, v: i32) -> Result<()> {
        let mem = get_memory(caller)?;
        let message = {
            let data = mem.data(&caller);
            read_arraybuffer(data, v)?
        };
        Ok(())
    }
}

pub struct AlkanesExportsImpl(());
impl AlkanesExportsImpl {
    pub fn _get_export(vm: &mut AlkanesInstance, name: &str) -> Result<Func> {
        let instance: &mut Instance = &mut vm.instance;
        let store: &mut Store<AlkanesState> = &mut vm.store;
        Ok(instance.get_func(store, name).ok_or("").map_err(|_| {
            anyhow!(format!(
                "{} not found -- is this WASM built with the ALKANES SDK?",
                name
            ))
        })?)
    }
    pub fn _get_result(vm: &mut AlkanesInstance, result: &[Val; 1]) -> Result<Vec<u8>> {
        vm.read_arraybuffer(
            result[0]
                .i32()
                .ok_or("")
                .map_err(|_| anyhow!("result is not an i32"))?,
        )
    }

    pub fn execute(vm: &mut AlkanesInstance) -> Result<ExtendedCallResponse> {
        let mut result = [Val::I32(0)];
        let func = Self::_get_export(vm, "__execute")?;
        func.call(&mut vm.store, &[], &mut result)?;
        println!("call");
        let response = ExtendedCallResponse::parse(&mut std::io::Cursor::new(Self::_get_result(
            vm, &result,
        )?))?;
        println!("response: {:?}", response);
        Ok(response)
    }
}

impl AlkanesInstance {
    pub fn consume_fuel(&mut self, fuel: u64) -> Result<()> {
        let fuel_remaining = self.store.get_fuel().unwrap();
        if fuel_remaining < fuel {
            Err(anyhow!(format!(
                "{} gas remaining but {} consumed by call",
                fuel_remaining, fuel
            )))
        } else {
            self.store.set_fuel(fuel_remaining - fuel).unwrap();
            Ok(())
        }
    }
    pub fn read_arraybuffer(&mut self, data_start: i32) -> anyhow::Result<Vec<u8>> {
        read_arraybuffer(self.get_memory()?.data(&self.store), data_start)
    }
    pub fn get_memory(&mut self) -> anyhow::Result<Memory> {
        self.instance
            .get_memory(&mut self.store, "memory")
            .ok_or("")
            .map_err(|_| anyhow!("memory segment not found"))
    }
    pub fn send_to_arraybuffer(&mut self, ptr: usize, v: &Vec<u8>) -> anyhow::Result<i32> {
        let mem = self.get_memory()?;
        mem.write(&mut self.store, ptr, &v.len().to_le_bytes())
            .map_err(|_| anyhow!("failed to write ArrayBuffer"))?;
        mem.write(&mut self.store, ptr + 4, v.as_slice())
            .map_err(|_| anyhow!("failed to write ArrayBuffer"))?;
        Ok((ptr + 4).try_into()?)
    }
    pub fn checkpoint(&mut self) {
        (&mut self.store.data_mut().context.lock().unwrap().message)
            .atomic
            .checkpoint();
    }
    pub fn commit(&mut self) {
        (&mut self.store.data_mut().context.lock().unwrap().message)
            .atomic
            .commit();
    }
    pub fn rollback(&mut self) {
        (&mut self.store.data_mut().context.lock().unwrap().message)
            .atomic
            .rollback();
    }
    pub fn from_alkane(context: AlkanesRuntimeContext, start_fuel: u64) -> Result<Self> {
        let binary = context
            .message
            .atomic
            .keyword("/alkanes/")
            .select(&context.myself.clone().into())
            .get();
        let mut config = Config::default();
        config.consume_fuel(true);
        let engine = Engine::new(&config);
        let mut store = Store::<AlkanesState>::new(
            &engine,
            AlkanesState {
                had_failure: false,
                limiter: StoreLimitsBuilder::new().memory_size(MEMORY_LIMIT).build(),
                context: Arc::new(Mutex::new(context)),
            },
        );
        store.limiter(|state| &mut state.limiter);
        Store::<AlkanesState>::set_fuel(&mut store, start_fuel)?; // TODO: implement gas limits
        let module = Module::new(&engine, &mut &binary[..])?;
        let mut linker: Linker<AlkanesState> = Linker::<AlkanesState>::new(&engine);
        linker.func_wrap("env", "abort", AlkanesHostFunctionsImpl::abort)?;
        linker.func_wrap(
            "env",
            "__load_storage",
            |mut caller: Caller<'_, AlkanesState>, k: i32, v: i32| {
                match AlkanesHostFunctionsImpl::load_storage(&mut caller, k, v) {
                    Ok(v) => v,
                    Err(_e) => {
                        AlkanesHostFunctionsImpl::_abort(caller);
                        -1
                    }
                }
            },
        )?;
        linker.func_wrap(
            "env",
            "__request_storage",
            |mut caller: Caller<'_, AlkanesState>, k: i32| {
                match AlkanesHostFunctionsImpl::request_storage(&mut caller, k) {
                    Ok(v) => v,
                    Err(_e) => {
                        AlkanesHostFunctionsImpl::_abort(caller);
                        -1
                    }
                }
            },
        )?;
        linker.func_wrap(
            "env",
            "__log",
            |mut caller: Caller<'_, AlkanesState>, v: i32| {
                if let Err(_e) = AlkanesHostFunctionsImpl::log(&mut caller, v) {
                    AlkanesHostFunctionsImpl::_abort(caller);
                }
            },
        )?;
        linker.func_wrap(
            "env",
            "__balance",
            |mut caller: Caller<'_, AlkanesState>, who: i32, what: i32, output: i32| {
                if let Err(_e) = AlkanesHostFunctionsImpl::balance(&mut caller, who, what, output) {
                    AlkanesHostFunctionsImpl::_abort(caller);
                }
            },
        )?;
        linker.func_wrap(
            "env",
            "__request_context",
            |mut caller: Caller<'_, AlkanesState>| -> i32 {
                match AlkanesHostFunctionsImpl::request_context(&mut caller) {
                    Ok(v) => v,
                    Err(_e) => {
                        AlkanesHostFunctionsImpl::_abort(caller);
                        -1
                    }
                }
            },
        )?;
        linker.func_wrap(
            "env",
            "__load_context",
            |mut caller: Caller<'_, AlkanesState>, output: i32| {
                match AlkanesHostFunctionsImpl::load_context(&mut caller, output) {
                    Ok(v) => v,
                    Err(_e) => {
                        AlkanesHostFunctionsImpl::_abort(caller);
                        -1
                    }
                }
            },
        )?;
        linker.func_wrap(
            "env",
            "__sequence",
            |mut caller: Caller<'_, AlkanesState>, output: i32| {
                if let Err(_e) = AlkanesHostFunctionsImpl::sequence(&mut caller, output) {
                    AlkanesHostFunctionsImpl::_abort(caller);
                }
            },
        )?;
        linker.func_wrap(
            "env",
            "__fuel",
            |mut caller: Caller<'_, AlkanesState>, output: i32| {
                if let Err(_e) = AlkanesHostFunctionsImpl::fuel(&mut caller, output) {
                    AlkanesHostFunctionsImpl::_abort(caller);
                }
            },
        )?;

        linker.func_wrap(
            "env",
            "__returndatacopy",
            |mut caller: Caller<'_, AlkanesState>, output: i32| {
                if let Err(_e) = AlkanesHostFunctionsImpl::returndatacopy(&mut caller, output) {
                    AlkanesHostFunctionsImpl::_abort(caller);
                }
            },
        )?;
        linker.func_wrap(
            "env",
            "__request_transaction",
            |mut caller: Caller<'_, AlkanesState>| {
                match AlkanesHostFunctionsImpl::request_transaction(&mut caller) {
                    Ok(v) => v,
                    Err(_e) => {
                        AlkanesHostFunctionsImpl::_abort(caller);
                        -1
                    }
                }
            },
        )?;
        linker.func_wrap(
            "env",
            "__load_transaction",
            |mut caller: Caller<'_, AlkanesState>, output: i32| {
                if let Err(_e) = AlkanesHostFunctionsImpl::load_transaction(&mut caller, output) {
                    AlkanesHostFunctionsImpl::_abort(caller);
                }
            },
        )?;
        linker.func_wrap(
            "env",
            "__request_block",
            |mut caller: Caller<'_, AlkanesState>| match AlkanesHostFunctionsImpl::request_block(
                &mut caller,
            ) {
                Ok(v) => v,
                Err(_e) => {
                    AlkanesHostFunctionsImpl::_abort(caller);
                    -1
                }
            },
        )?;
        linker.func_wrap(
            "env",
            "__load_block",
            |mut caller: Caller<'_, AlkanesState>, output: i32| {
                if let Err(_e) = AlkanesHostFunctionsImpl::load_block(&mut caller, output) {
                    AlkanesHostFunctionsImpl::_abort(caller);
                }
            },
        )?;
        linker.func_wrap(
            "env",
            "__call",
            |mut caller: Caller<'_, AlkanesState>,
             cellpack_ptr: i32,
             incoming_alkanes_ptr: i32,
             checkpoint_ptr: i32,
             start_fuel: u64|
             -> i32 {
                match AlkanesHostFunctionsImpl::extcall::<Call>(
                    &mut caller,
                    cellpack_ptr,
                    incoming_alkanes_ptr,
                    checkpoint_ptr,
                    start_fuel,
                ) {
                    Ok(v) => v,
                    Err(_e) => {
                        AlkanesHostFunctionsImpl::_abort(caller);
                        -1
                    }
                }
            },
        )?;
        linker.func_wrap(
            "env",
            "__delegatecall",
            |mut caller: Caller<'_, AlkanesState>,
             cellpack_ptr: i32,
             incoming_alkanes_ptr: i32,
             checkpoint_ptr: i32,
             start_fuel: u64|
             -> i32 {
                match AlkanesHostFunctionsImpl::extcall::<Delegatecall>(
                    &mut caller,
                    cellpack_ptr,
                    incoming_alkanes_ptr,
                    checkpoint_ptr,
                    start_fuel,
                ) {
                    Ok(v) => v,
                    Err(_e) => {
                        AlkanesHostFunctionsImpl::_abort(caller);
                        -1
                    }
                }
            },
        )?;
        linker.func_wrap(
            "env",
            "__staticcall",
            |mut caller: Caller<'_, AlkanesState>,
             cellpack_ptr: i32,
             incoming_alkanes_ptr: i32,
             checkpoint_ptr: i32,
             start_fuel: u64|
             -> i32 {
                match AlkanesHostFunctionsImpl::extcall::<Staticcall>(
                    &mut caller,
                    cellpack_ptr,
                    incoming_alkanes_ptr,
                    checkpoint_ptr,
                    start_fuel,
                ) {
                    Ok(v) => v,
                    Err(_e) => {
                        AlkanesHostFunctionsImpl::_abort(caller);
                        -1
                    }
                }
            },
        )?;
        Ok(AlkanesInstance {
            instance: linker
                .instantiate(&mut store, &module)?
                .ensure_no_start(&mut store)?,
            store,
        })
    }
    pub fn reset(&mut self) {
        self.store.data_mut().had_failure = false;
    }
    pub fn execute(&mut self) -> Result<ExtendedCallResponse> {
        self.checkpoint();
        let (call_response, had_failure): (ExtendedCallResponse, bool) = {
            match AlkanesExportsImpl::execute(self) {
                Ok(v) => {
                    if self.store.data().had_failure {
                        (ExtendedCallResponse::default(), true)
                    } else {
                        (v, false)
                    }
                }
                Err(e) => {
                    println!("error: {}", e);
                    (ExtendedCallResponse::default(), true)
                }
            }
        };
        self.reset();
        if had_failure {
            self.rollback();
            Err(anyhow!("ALKANES: revert"))
        } else {
            self.commit();
            Ok(call_response)
        }
    }
}

pub fn sequence_pointer(ptr: &AtomicPointer) -> AtomicPointer {
    ptr.derive(&IndexPointer::from_keyword("/alkanes/sequence"))
}

pub fn run_special_cellpacks(
    context: &mut AlkanesRuntimeContext,
    cellpack: &Cellpack,
) -> Result<(AlkaneId, AlkaneId)> {
    let mut payload = cellpack.clone();
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
        context
            .message
            .atomic
            .keyword("/alkanes/")
            .select(&payload.target.clone().into())
            .set(wasm_payload.clone());
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
    } else if let Some(factory) = cellpack.target.factory() {
        let mut next_sequence_pointer = sequence_pointer(&context.message.atomic);
        let next_sequence = next_sequence_pointer.get_value::<u128>();
        payload.target = AlkaneId::new(2, next_sequence);
        next_sequence_pointer.set_value(next_sequence + 1);
        let binary: Vec<u8> = context
            .message
            .atomic
            .keyword("/alkanes/")
            .select(&factory.clone().into())
            .get()
            .as_ref()
            .clone();
        context
            .message
            .atomic
            .keyword("/alkanes/")
            .select(&payload.target.clone().into())
            .set(Arc::new(binary));
    }
    Ok((context.myself.clone(), payload.target.clone()))
}

pub fn run_after_special(
    context: AlkanesRuntimeContext,
    start_fuel: u64,
) -> Result<ExtendedCallResponse> {
    Ok(AlkanesInstance::from_alkane(context, start_fuel)?.execute()?)
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
) -> Result<ExtendedCallResponse> {
    let (caller, myself) = run_special_cellpacks(&mut context, cellpack)?;
    prepare_context(&mut context, &caller, &myself, delegate);
    run_after_special(context, start_fuel)
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
