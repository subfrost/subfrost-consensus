#[link(wasm_import_module = "env")]
extern "C" {
    pub fn __log(v: i32);
}
