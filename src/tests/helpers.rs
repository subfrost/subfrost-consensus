use anyhow::{anyhow, Result};
use js_sys::{Promise, Uint8Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

#[wasm_bindgen(module = "src/tests/utils.js")]
extern "C" {
    #[wasm_bindgen(js_name = "fetchWasmFile")]
    fn fetch_wasm_file(url: &str) -> Promise;
}

pub async fn read_sample_contract() -> Result<Vec<u8>> {
    let target_url = "./target/wasm32-unknown-unknown/debug/deps/sample-alkane.wasm";

    // Step 1: Fetch the WASM file
    let fetch_promise: Promise = fetch_wasm_file(target_url);
    let wasm_binary_jsvalue = JsFuture::from(fetch_promise).await.map_err(|e| {
        anyhow!(
            "Failed to fetch WASM file: {:?}",
            e.as_string().unwrap_or_else(|| "Unknown error".into())
        )
    })?;

    // Step 2: Convert the fetched value into a Response object
    let response: Response = wasm_binary_jsvalue
        .dyn_into()
        .map_err(|e| anyhow!("Failed to cast JsValue to Response: {:?}", e))?;

    // Step 3: Get the array buffer from the response and handle the future
    let wasm_bytes_jsvalue = JsFuture::from(response.array_buffer().map_err(|e| {
        anyhow!(
            "Failed to get array buffer: {:?}",
            e.as_string().unwrap_or_else(|| "Unknown error".into())
        )
    })?)
    .await
    .map_err(|e| {
        anyhow!(
            "Failed to resolve array buffer future: {:?}",
            e.as_string().unwrap_or_else(|| "Unknown error".into())
        )
    })?;

    // Step 4: Convert the array buffer to a Vec<u8>
    let wasm_bytes = Uint8Array::new(&wasm_bytes_jsvalue).to_vec();

    Ok(wasm_bytes)
}
