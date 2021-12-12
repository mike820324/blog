use wasm_bindgen::prelude::*;

// wasm-bindgen will automatically take care of including this script
#[wasm_bindgen(module = "/src/binding/highlight.js")]
extern "C" {
    #[wasm_bindgen(js_name = "highlightAll")]
    pub fn highlight_all() -> bool;
}