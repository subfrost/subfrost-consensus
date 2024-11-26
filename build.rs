use anyhow::Result;
use flate2::write::GzEncoder;
use flate2::Compression;
use hex;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::process::{Command, Stdio};

fn compress(binary: Vec<u8>) -> Result<Vec<u8>> {
    let mut writer = GzEncoder::new(Vec::<u8>::with_capacity(binary.len()), Compression::best());
    writer.write_all(&binary)?;
    Ok(writer.finish()?)
}

fn build_alkane(wasm_str: &str, features: Vec<&'static str>) -> Result<()> {
    if features.len() != 0 {
      let _ = Command::new("cargo")
        .env("CARGO_TARGET_DIR", wasm_str)
        .arg("build")
        .arg("--release")
        .arg("--features")
        .arg(features.join(","))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;
      Ok(())
    } else {
      Command::new("cargo")
        .env("CARGO_TARGET_DIR", wasm_str)
        .arg("build")
        .arg("--release")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;
      Ok(())
    }
}

fn main() {
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
    let mods = fs::read_dir(&crates_dir)
        .unwrap()
        .filter_map(|v| {
            let name = v.ok()?.file_name().into_string().ok()?;
            if name.starts_with("alkanes-std-") {
                Some(name)
            } else {
                None
            }
        })
        .collect::<Vec<String>>();
    let files = mods
        .clone()
        .into_iter()
        .filter_map(|name| {
            if let Some(feature_name) = name.strip_prefix("alkanes-std-") {
                let final_name = feature_name.to_uppercase().replace("-", "_");
                if let Some(_) = env::var(format!("CARGO_FEATURE_{}", final_name.as_str())).ok() {
                    Some(name)
                } else if let Some(_) = env::var(format!("CARGO_FEATURE_{}", "ALL")).ok() {
                    Some(name)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<String>>();
    files.into_iter()
        .map(|v| -> Result<String> {
            std::env::set_current_dir(&crates_dir.clone().join(v.clone()))?;
            if v == "alkanes-std-genesis-alkane" {
              if let Some(_) = env::var("CARGO_FEATURE_REGTEST").ok() {
                build_alkane(wasm_str, vec!["regtest"])?;
              } else if let Some(_) = env::var("CARGO_FEATURE_MAINNET").ok() {
                build_alkane(wasm_str, vec!["mainnet"])?;
              } else if let Some(_) = env::var("CARGO_FEATURE_DOGECOIN").ok() {
                build_alkane(wasm_str, vec!["dogecoin"])?;
              } else if let Some(_) = env::var("CARGO_FEATURE_FRACTAL").ok() {
                build_alkane(wasm_str, vec!["fractal"])?;
              } else if let Some(_) = env::var("CARGO_FEATURE_LUCKYCOIN").ok() {
                build_alkane(wasm_str, vec!["luckycoin"])?;
              } else if let Some(_) = env::var("CARGO_FEATURE_BELLSCOIN").ok() {
                build_alkane(wasm_str, vec!["bellscoin"])?;
              }
            } else {
              build_alkane(wasm_str, vec![])?;
            }
            std::env::set_current_dir(&crates_dir)?;
            let subbed = v.clone().replace("-", "_");
            let f: Vec<u8> = fs::read(
                &Path::new(&wasm_str)
                    .join("wasm32-unknown-unknown")
                    .join("release")
                    .join(subbed.clone() + ".wasm"),
            )?;
            let data: String = hex::encode(&f);
            eprintln!(
                "write: {}",
                write_dir
                    .join("std")
                    .join(subbed.clone() + "_build.rs")
                    .into_os_string()
                    .to_str()
                    .unwrap()
            );
            fs::write(&Path::new(&wasm_str).join("wasm32-unknown-unknown").join("release").join(subbed.clone() + ".wasm.gz"), &compress(f.clone())?)?;
            fs::write(
                &write_dir.join("std").join(subbed.clone() + "_build.rs"),
                String::from("use hex_lit::hex;\n#[allow(long_running_const_eval)]\npub fn get_bytes() -> Vec<u8> { (&hex!(\"")
                    + data.as_str()
                    + "\")).to_vec() }",
            )?;
            eprintln!(
                "build: {}",
                write_dir
                    .join("std")
                    .join(subbed.clone() + "_build.rs")
                    .into_os_string()
                    .to_str()
                    .unwrap()
            );
            Ok(subbed)
        })
        .collect::<Result<Vec<String>>>()
        .unwrap();
    eprintln!(
        "write test builds to: {}",
        write_dir
            .join("std")
            .join("mod.rs")
            .into_os_string()
            .to_str()
            .unwrap()
    );
    fs::write(
        &write_dir.join("std").join("mod.rs"),
        mods.into_iter()
            .map(|v| v.replace("-", "_"))
            .fold(String::default(), |r, v| {
                r + "pub mod " + v.as_str() + "_build;\n"
            }),
    )
    .unwrap();
}
