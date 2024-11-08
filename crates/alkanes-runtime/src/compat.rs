use crate::{println, stdio::stdout};
use std::fmt::Write;
use std::panic;
pub fn panic_hook(info: &panic::PanicHookInfo) {
    println!("panicked with error: {}", info.to_string());
}
