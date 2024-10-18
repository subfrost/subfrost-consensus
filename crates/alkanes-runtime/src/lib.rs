pub mod imports;
pub mod runtime;
pub mod stdio;
pub use crate::stdio::stdout;

#[no_mangle]
fn __execute() -> i32 {
    0
}
