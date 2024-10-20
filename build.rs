use hex;
use protobuf_codegen;
use protoc_bin_vendored;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
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
    let out_dir = base_dir
        .join("release");
    let out_str = out_dir.to_str().unwrap();
    let write_dir = Path::new(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("src")
        .join("tests")
        .join("sample_alkane.rs");
    std::env::set_current_dir(&out_dir.parent().unwrap().parent().unwrap().parent().unwrap().join("crates").join("sample-alkane")).unwrap();
    Command::new("cargo").arg("build").arg("--release").spawn().expect("failed to execute cargo to build test alkanes").wait().expect("failed to wait on cargo build");
    let data: String =
        hex::encode(&fs::read(&Path::new(&out_str).join("sample_alkane.wasm")).unwrap());
    fs::write(
        &write_dir,
        String::from("use hex_lit::hex;\npub fn get_bytes() -> Vec<u8> { (&hex!(\"")
            + data.as_str()
            + "\")).to_vec() }",
    )
    .unwrap();
}
