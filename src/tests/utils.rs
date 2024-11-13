#[cfg(test)]
mod tests {
    use alkanes_support::response::ExtendedCallResponse;
    use anyhow::Result;
    #[allow(unused_imports)]
    use metashrew::{
        clear,
        index_pointer::IndexPointer,
        println,
        stdio::{stdout, Write},
    };
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_response_serialization() -> Result<()> {
        clear();
        Ok(())
    }
}
