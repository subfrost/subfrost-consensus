use crate::vm::{AlkanesInstance, AlkanesState};
use alkanes_support::utils::overflow_error;
use anyhow::Result;
use wasmi::*;

#[cfg(feature = "mainnet")]
const TOTAL_FUEL: u64 = 100_000_000;
#[cfg(feature = "dogecoin")]
const TOTAL_FUEL: u64 = 60_000_000;
#[cfg(feature = "fractal")]
const TOTAL_FUEL: u64 = 50_000_000;
#[cfg(feature = "luckycoin")]
const TOTAL_FUEL: u64 = 50_000_000;
#[cfg(feature = "bellscoin")]
const TOTAL_FUEL: u64 = 50_000_000;

static mut MESSAGE_COUNT: u64 = 0;

pub const FUEL_PER_REQUEST_BYTE: u64 = 1;
pub const FUEL_PER_LOAD_BYTE: u64 = 2;
pub const FUEL_PER_STORE_BYTE: u64 = 8;
pub const FUEL_SEQUENCE: u64 = 5;
pub const FUEL_FUEL: u64 = 5;
pub const FUEL_EXTCALL: u64 = 500;
pub const FUEL_HEIGHT: u64 = 10;
pub const FUEL_BALANCE: u64 = 10;
pub const FUEL_EXTCALL_DEPLOY: u64 = 10_000;

pub trait Fuelable {
    fn consume_fuel(&mut self, n: u64) -> Result<()>;
}

impl<'a> Fuelable for Caller<'_, AlkanesState> {
    fn consume_fuel(&mut self, n: u64) -> Result<()> {
        overflow_error((self.get_fuel().unwrap() as u64).checked_sub(n))?;
        Ok(())
    }
}

impl Fuelable for AlkanesInstance {
    fn consume_fuel(&mut self, n: u64) -> Result<()> {
        overflow_error((self.store.get_fuel().unwrap() as u64).checked_sub(n))?;
        Ok(())
    }
}

pub fn consume_fuel<'a>(caller: &mut Caller<'_, AlkanesState>, n: u64) -> Result<()> {
    caller.consume_fuel(n)
}

pub fn set_message_count(v: u64) {
    unsafe {
        MESSAGE_COUNT = v;
    }
}

pub fn start_fuel() -> u64 {
    TOTAL_FUEL / std::cmp::max(1, unsafe { MESSAGE_COUNT })
}

pub fn compute_extcall_fuel(savecount: u64) -> Result<u64> {
    let save_fuel = overflow_error(FUEL_PER_STORE_BYTE.checked_mul(savecount))?;
    overflow_error::<u64>(FUEL_EXTCALL.checked_add(save_fuel))
}
