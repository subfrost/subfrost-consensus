const TOTAL_FUEL: u64 = 100_000_000;
use std::fmt::{Write};

static mut MESSAGE_COUNT: u64 = 0;

pub fn set_message_count(v: u64) {
  unsafe { MESSAGE_COUNT = v; }
}

pub fn start_fuel() -> u64 {
  TOTAL_FUEL / std::cmp::max(1, unsafe { MESSAGE_COUNT })
}
