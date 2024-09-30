use wasmi::*;
use bitcoin::blockdata::{block::Block, transaction::Transaction};
use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::sync::{Arc, Mutex};
use wasmi::*;

#[derive(Default, Clone)]
struct AlkaneRuntimeContext {
    pub myself: AlkaneId,
    pub caller: AlkaneId,
    pub incoming_alkanes: AlkaneTransferParcel,
    pub state: AlkaneGlobalState,
    pub returndata: Vec<u8>,
    pub inputs: Vec<u128>
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

pub struct AlkaneState {
  pub had_failure: bool,
  pub context: Arc<Mutex<AlkaneRuntimeContext>>,
  pub limiter: StoreLimits,
}

pub struct AlkaneInstance {
  instance: Instance,
  store: Store<AlkaneState>,
}


pub fn get_memory<'a>(caller: &mut Caller<'_, AlkaneState>) -> Result<Memory> {
    caller
        .get_export("memory")
        .ok_or(anyhow!("export was not memory region"))?
        .into_memory()
        .ok_or(anyhow!("export was not memory region"))
}

const MEMORY_LIMIT: usize = 33554432;

impl AlkanesHostFunctionsImpl {
    fn _abort<'a>(caller: Caller<'_, AlkaneState>) {
        AlkanesHostFunctionsImpl::abort(caller, 0, 0, 0, 0);
    }
    fn abort<'a>(mut caller: Caller<'_, AlkaneState>, _: i32, _: i32, _: i32, _: i32) {
        caller.data_mut().had_failure = true;
    }
    fn load_storage<'a>(caller: &mut Caller<'_, AlkaneState>, k: i32, v: i32) -> Result<i32> {
        let mem = get_memory(caller)?;
        let key = {
            let data = mem.data(&caller);
            read_arraybuffer(data, k)?
        };
        let value = caller
            .data_mut()
            .storage
            .lock()
            .unwrap()
            .contract
            .get(&(&key.to_be_bytes::<32>()).try_into()?);
        send_to_arraybuffer(caller, &value)
    }
    fn call<'a>(caller: &mut Caller<'_, AlkaneState>, data: i32) -> Result<i32> {
        let buffer = read_arraybuffer(get_memory(caller)?.data(&caller), data)?;
        let mut reader = BytesReader::from(&buffer);
        let (contract_address, calldata): (Vec<u8>, Vec<u8>) = (
            reader.read_address()?.as_str().as_bytes().to_vec(),
            reader.read_bytes_with_length()?,
        );
        if let Some(_v) = caller.data().call_stack.get(&contract_address) {
            return Err(anyhow!("failure -- reentrancy guard"));
        }
        let mut vm = AlkanesContract::get(
            &contract_address,
            Arc::new(Mutex::new(
                caller.data_mut().storage.lock().unwrap().clone(),
            )),
        )?
        .ok_or("")
        .map_err(|_| match String::from_utf8(contract_address.clone()) {
            Ok(v) => anyhow!(format!(
                "failed to call non-existent contract at address {}",
                v
            )),
            Err(_) => anyhow!("failed to convert contract address from utf-8"),
        })?;
        {
            let mut environment = caller.data_mut().environment.clone();
            environment.set_contract_address(&contract_address.clone());
            vm.store.data_mut().environment = environment;
        }
        AlkanesExportsImpl::set_environment(&mut vm)?;
        let call_response = vm.run(calldata)?;
        vm.store.data_mut().storage = call_response.storage.clone();
        vm.store.data_mut().events = call_response.events;
        vm.consume_fuel(call_response.gas_used)?;
        send_to_arraybuffer(caller, &call_response.response)
        // TODO: encode response
    }
    fn log<'a>(caller: &mut Caller<'_, AlkaneState>, v: i32) -> Result<()> {
        crate::stdio::log({
            let mem = get_memory(caller)?;
            Arc::new(read_arraybuffer(mem.data(&caller), v)?)
        });
        Ok(())
    }
}

impl AlkanesExportsImpl {
    pub fn _get_export(vm: &mut AlkanesContract, name: &str) -> Result<Func> {
        let instance: &mut Instance = &mut vm.instance;
        let store: &mut Store<State> = &mut vm.store;
        Ok(instance.get_func(store, name).ok_or("").map_err(|_| {
            anyhow!(format!(
                "{} not found -- is this WASM built with the ALKANES SDK?",
                name
            ))
        })?)
    }
    pub fn _get_result(vm: &mut AlkanesContract, result: &[Val; 1]) -> Result<Vec<u8>> {
        vm.read_arraybuffer(
            result[0]
                .i32()
                .ok_or("")
                .map_err(|_| anyhow!("result is not an i32"))?,
        )
    }
    pub fn execute(vm: &mut AlkanesContract) -> Result<Vec<u8>> {
        let mut result = [Val::I32(0)];
        let func = Self::_get_export(vm, "__execute")?;
        let arg2 = vm.send_to_arraybuffer(data)?;
        func.call(
            &mut vm.store,
            &[Val::I32(method as i32), Val::I32(arg2)],
            &mut result,
        )?;
        Self::_get_result(vm, &result)
    }
}

impl AlkaneContract {
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
    pub fn get(address: &Vec<u8>, storage: Arc<Mutex<StorageView>>) -> Result<Option<Self>> {
        let saved = IndexPointer::from_keyword("/alkanes/")
            .select(address)
            .get();
        if saved.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(Self::load_from_address(address, &saved, storage)?))
        }
    }
    pub fn checkpoint(&mut self) -> Arc<Mutex<StorageView>> {
        Arc::new(Mutex::new(
            self.store.data_mut().storage.lock().unwrap().clone(),
        ))
    }
    pub fn load(address: &Vec<u8>, program: &Vec<u8>) -> Result<Self> {
        let mut storage = StorageView::default();
        storage.global.lazy_load(address);
        storage.contract = storage.global.0.get(address).clone().unwrap().clone();
        return Self::load_from_address(address, program, Arc::new(Mutex::new(storage)));
    }
    pub fn load_from_address(
        address: &Vec<u8>,
        program: &Vec<u8>,
        storage: Arc<Mutex<StorageView>>,
    ) -> Result<Self> {
        let mut config = Config::default();
        config.consume_fuel(true);
        let engine = Engine::new(&config);
        let mut store = Store::<State>::new(
            &engine,
            AlkanesState {
                environment: AlkanesEnvironment::default(),
                events: vec![],
                had_failure: false,
                limiter: StoreLimitsBuilder::new().memory_size(MEMORY_LIMIT).build(),
                call_stack: HashSet::<Vec<u8>>::new(),
                storage: storage.clone(),
            },
        );
        store.limiter(|state| &mut state.limiter);
        Store::<State>::set_fuel(&mut store, 100000)?; // TODO: implement gas limits
        let cloned = program.clone();
        let module = Module::new(&engine, &mut &cloned[..])?;
        let mut linker: Linker<State> = Linker::<State>::new(&engine);
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
        Ok(AlkaneContract {
            instance: linker
                .instantiate(&mut store, &module)?
                .ensure_no_start(&mut store)?,
            store,
            storage,
        })
    }
    pub fn reset(&mut self) {
        self.store.data_mut().had_failure = false;
    }
    pub fn run(&mut self, payload: Vec<u8>) -> Result<CallResponse, anyhow::Error> {
        let start_fuel = self.store.get_fuel()?;
        let call_response: Vec<u8> =lkanesExportsImpl::execute(self)?
        let had_failure = self.store.data().had_failure;
        self.reset();
        if had_failure {
            Err(anyhow!("ALKANES: revert"))
        } else {
            let checkpoint = { self.checkpoint() };
            let mut state: &mut AlkanesState = self.store.data_mut();
            state.storage.lock().unwrap().commit();
            state.storage = checkpoint.clone();
            Ok(CallResponse {
                storage: checkpoint.clone(),
                response: call_response,
                gas_used: start_fuel - self.store.get_fuel()?,
            })
        }
    }
}

pub fn send_to_arraybuffer<'a>(caller: &mut Caller<'_, AlkaneState>, ptr: usize, v: &Vec<u8>) -> Result<i32> {
    let mem = get_memory(caller)?;
    mem.write(&mut *caller, ptr - 4, &v.len().to_le_bytes())
        .map_err(|_| anyhow!("failed to write ArrayBuffer"))?;
    mem.write(&mut *caller, ptr, v.as_slice())
        .map_err(|_| anyhow!("failed to write ArrayBuffer"))?;
    Ok(ptr)
}
