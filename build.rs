use anyhow::Result;
use hex;
use protobuf_codegen;
use protoc_bin_vendored;
use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
fn main() {
    protobuf_codegen::Codegen::new()
        .protoc()
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        .out_dir("src/proto")
        .inputs(&["proto/protorune.proto"])
        .include("proto")
        .run()
        .expect("running protoc failed");
    let env_var = env::var_os("OUT_DIR").unwrap();
    let base_dir = Path::new(&env_var)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let out_dir = base_dir.join("release");
    let wasm_dir = base_dir.parent().unwrap().join("alkanes");
    fs::create_dir_all(&wasm_dir).unwrap();
    let wasm_str = wasm_dir.to_str().unwrap();
    let write_dir = Path::new(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("src")
        .join("tests");

    fs::create_dir_all(&write_dir.join("std")).unwrap();
    let crates_dir = out_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("crates");
    std::env::set_current_dir(&crates_dir).unwrap();
    let files = fs::read_dir(&crates_dir)
        .unwrap()
        .filter_map(|v| {
            let name = v.ok()?.file_name().into_string().ok()?;
            if name.starts_with("alkanes-std-") {
                Some(name)
            } else {
                None
            }
        })
        .map(|v| -> Result<String> {
            std::env::set_current_dir(&crates_dir.clone().join(v.clone()))?;
            Command::new("cargo")
                .env("CARGO_TARGET_DIR", wasm_str)
                .arg("build")
                .arg("--release")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("failed to execute cargo to build test alkanes")
                .wait()
                .expect("failed to wait on build job");
            std::env::set_current_dir(&crates_dir)?;
            let subbed = v.clone().replace("-", "_");
            let data: String = hex::encode(&fs::read(
                &Path::new(&wasm_str).join("wasm32-unknown-unknown").join("release").join(subbed.clone() + ".wasm")
            )?);
            fs::write(
                &write_dir.join("std").join(subbed.clone() + "_build.rs"),
                String::from("use hex_lit::hex;\npub fn get_bytes() -> Vec<u8> { (&hex!(\"")
                    + data.as_str()
                    + "\")).to_vec() }",
            )?;
            Ok(subbed)
        })
        .collect::<Result<Vec<String>>>()
        .unwrap();
    fs::write(
        &write_dir.join("std").join("mod.rs"),
        files.into_iter().fold(String::default(), |r, v| {
            r + "pub mod " + v.as_str() + "_build;\n"
        }),
    )
    .unwrap();
}
