use crate::{println, stdio::stdout};
use std::fmt::Write;
use std::panic;
pub fn panic_hook(info: &panic::PanicHookInfo) {
    println!("panic! within WASM: {}", info.to_string());
}
