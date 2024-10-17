use anyhow::Result;
use std::{fs, process::Command};

pub fn read_sample_contract() -> Result<Vec<u8>> {
    let lib_path = "./sample_contract";
    let target_dir = "./sample_contract/target/wasm32-unknown-unknown/debug";

    // Run `cargo build` for the wasm_lib targeting WASM.
    let status = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .current_dir(lib_path)
        .status()?;

    println!("built");

    // Check if the build was successful.
    if !status.success() {
        panic!("Failed to compile the wasm_lib to WebAssembly");
    }

    // Step 2: Load the compiled `.wasm` file into memory.
    let wasm_file = format!("{}/sample_contract.wasm", target_dir);
    let wasm_binary = fs::read(wasm_file)?;
    Ok(wasm_binary)
}
