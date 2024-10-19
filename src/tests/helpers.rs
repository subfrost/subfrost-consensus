use anyhow::Result;
use std::fs;

pub fn read_sample_contract() -> Result<Vec<u8>> {
    let target_dir = "./target/wasm32-unknown-unknown/debug/deps";

    // Step 2: Load the compiled `.wasm` file into memory.
    let wasm_file = format!("{}/sample-alkane.wasm", target_dir);
    let wasm_binary = fs::read(wasm_file)?;
    Ok(wasm_binary)
}
