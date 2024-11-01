use super::{AlkanesInstance, AlkanesState};
use alkanes_support::response::ExtendedCallResponse;
use anyhow::{anyhow, Result};
use wasmi::*;

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
        let response = ExtendedCallResponse::parse(&mut std::io::Cursor::new(Self::_get_result(
            vm, &result,
        )?))?;
        Ok(response)
    }
}
