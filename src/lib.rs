use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::decode;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) {
    // log(&encoded_file.into());
    log(&"Grayscale function called".into());

    let base64_to_vector = decode(encoded_file).expect("Failed to decode base64 string");
    log(&"Image decoded from base64".into());
}