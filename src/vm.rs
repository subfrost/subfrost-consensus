use wasmi::*;
use bitcoin::blockdata::{block::Block, transaction::Transaction};
use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};
use protorune::message::{MessageContext, MessageContextParcel};
use metashrew::{index_pointer::{KeyValuePointer, IndexPointer, AtomicPointer}, println, stdio::{stdout}};
use crate::{
  response::{CallResponse},
  storage::{StorageMap},
  parcel::{AlkaneTransfer, AlkaneTransferParcel},
  utils::{consume_to_end, consume_sized_int},
  message::{AlkaneMessageContext},
  id::{AlkaneId}
};
use std::fmt::{Write};
use wasmi::*;

#[derive(Default, Clone)]
struct AlkanesRuntimeContext {
    pub myself: AlkaneId,
    pub caller: AlkaneId,
    pub incoming_alkanes: AlkaneTransferParcel,
    pub returndata: Vec<u8>,
    pub inputs: Vec<u128>,
    pub message: Box<MessageContextParcel>
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
  store: Store<AlkanesState>
}


pub fn get_memory<'a>(caller: &mut Caller<'_, AlkanesState>) -> Result<Memory> {
    caller
        .get_export("memory")
        .ok_or(anyhow!("export was not memory region"))?
        .into_memory()
        .ok_or(anyhow!("export was not memory region"))
}

const MEMORY_LIMIT: usize = 33554432;

pub struct AlkanesHostFunctionsImpl(());
impl AlkanesHostFunctionsImpl {
    fn _abort<'a>(caller: Caller<'_, AlkanesState>) {
        AlkanesHostFunctionsImpl::abort(caller, 0, 0, 0, 0);
    }
    fn abort<'a>(mut caller: Caller<'_, AlkanesState>, _: i32, _: i32, _: i32, _: i32) {
        caller.data_mut().had_failure = true;
    }
    fn load_storage<'a>(caller: &mut Caller<'_, AlkanesState>, k: i32, v: i32) -> Result<i32> {
        let mem = get_memory(caller)?;
        let key = {
            let data = mem.data(&caller);
            read_arraybuffer(data, k)?
        };
        let value = {
          let myself = caller.data_mut().context.lock().unwrap().myself.clone();
          (&caller
            .data_mut()
            .context
            .lock()
            .unwrap()
            .message)
            .atomic
            .keyword("/alkanes/")
            .select(&myself.into())
            .keyword("/storage")
            .select(&key)
            .get()
        };
        send_to_arraybuffer(caller, v.try_into()?, value.as_ref())
    }
    fn call<'a>(caller: &mut Caller<'_, AlkanesState>, data: i32) -> Result<i32> {
        Ok(0)
        // TODO: call
    }
    fn log<'a>(caller: &mut Caller<'_, AlkanesState>, v: i32) -> Result<()> {
        let mem = get_memory(caller)?;
        let message = {
            let data = mem.data(&caller);
            read_arraybuffer(data, v)?
        };
        println!("{}", String::from_utf8(message)?);
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
    pub fn execute(vm: &mut AlkanesInstance) -> Result<CallResponse> {
        let mut result = [Val::I32(0)];
        let func = Self::_get_export(vm, "__execute")?;
        func.call(
            &mut vm.store,
            &[],
            &mut result,
        )?;
        Ok(CallResponse::parse(&mut std::io::Cursor::new(Self::_get_result(vm, &result)?))?)
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
    pub fn get(address: &Vec<u8>, context: Arc<Mutex<AlkanesRuntimeContext>>) -> Result<Option<Self>> {
        let saved = IndexPointer::from_keyword("/alkanes/")
            .select(address)
            .get();
        if saved.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(Self::load_from_context(address, &saved, context.clone())?))
        }
    }
    pub fn checkpoint(&mut self) {
        (&mut self.store.data_mut().context.lock().unwrap().message).atomic.checkpoint();
    }
    pub fn commit(&mut self) {
        (&mut self.store.data_mut().context.lock().unwrap().message).atomic.commit();
    }
    pub fn rollback(&mut self) {
        (&mut self.store.data_mut().context.lock().unwrap().message).atomic.rollback();
    }
    pub fn load_from_context(
        address: &Vec<u8>,
        program: &Vec<u8>,
        context: Arc<Mutex<AlkanesRuntimeContext>>
    ) -> Result<Self> {
        let mut config = Config::default();
        config.consume_fuel(true);
        let engine = Engine::new(&config);
        let mut store = Store::<AlkanesState>::new(
            &engine,
            AlkanesState {
                had_failure: false,
                limiter: StoreLimitsBuilder::new().memory_size(MEMORY_LIMIT).build(),
                context: context.clone()
            },
        );
        store.limiter(|state| &mut state.limiter);
        Store::<AlkanesState>::set_fuel(&mut store, 100000)?; // TODO: implement gas limits
        let cloned = program.clone();
        let module = Module::new(&engine, &mut &cloned[..])?;
        let mut linker: Linker<AlkanesState> = Linker::<AlkanesState>::new(&engine);
        linker.func_wrap("env", "abort", AlkanesHostFunctionsImpl::abort)?;
        linker.func_wrap("env", "__load_storage", |mut caller: Caller<'_, AlkanesState>, k: i32, v: i32| {
            match AlkanesHostFunctionsImpl::load_storage(&mut caller, k, v) {
                Ok(v) => v,
                Err(_e) => {
                    AlkanesHostFunctionsImpl::_abort(caller);
                    -1
                }
            }
        })?;
        linker.func_wrap("env", "__log", |mut caller: Caller<'_, AlkanesState>, v: i32| {
            if let Err(_e) = AlkanesHostFunctionsImpl::log(&mut caller, v) {
                AlkanesHostFunctionsImpl::_abort(caller);
            }
        })?;
        Ok(AlkanesInstance {
            instance: linker
                .instantiate(&mut store, &module)?
                .ensure_no_start(&mut store)?,
            store
        })
    }
    pub fn reset(&mut self) {
        self.store.data_mut().had_failure = false;
    }
    pub fn run(&mut self, payload: Vec<u8>) -> Result<CallResponse, anyhow::Error> {
        let start_fuel = self.store.get_fuel()?;
        self.checkpoint();
        let (call_response, had_failure): (CallResponse, bool) = {
          match AlkanesExportsImpl::execute(self) {
            Ok(v) => {
              if self.store.data().had_failure {
                (CallResponse::default(), true)
              } else {
                (v, false)
              }
            },
            Err(_) => (CallResponse::default(), true)
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

pub fn send_to_arraybuffer<'a>(caller: &mut Caller<'_, AlkanesState>, ptr: usize, v: &Vec<u8>) -> Result<i32> {
    let mem = get_memory(caller)?;
    mem.write(&mut *caller, ptr - 4, &v.len().to_le_bytes())
        .map_err(|_| anyhow!("failed to write ArrayBuffer"))?;
    mem.write(&mut *caller, ptr, v.as_slice())
        .map_err(|_| anyhow!("failed to write ArrayBuffer"))?;
    Ok(ptr.try_into()?)
}
