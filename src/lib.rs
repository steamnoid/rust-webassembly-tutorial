use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::{decode, encode};
use image::load_from_memory;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    // log(&encoded_file.into());
    log(&"Grayscale function called".into());

    let base64_to_vector = decode(encoded_file).expect("Failed to decode base64 string");
    log(&"Image decoded from base64".into());

    let mut img = load_from_memory(&base64_to_vector).expect("Failed to load image from memory");
    log(&"Image loaded from memory".into());

    img = img.grayscale();
    log(&"Image converted to grayscale".into());

    let mut buffer = vec![];
    img.write_to(&mut buffer, image::ImageOutputFormat::Png)
        .expect("Failed to write image to buffer");
    log(&"Image written to buffer".into());

    let encoded_img = encode(&buffer);
    log(&"Image encoded to base64".into());

    let data_url = format!("data:image/png;base64,{}", encoded_img);
    log(&"Data URL created".into());
    // log(&encoded_img.into());

    data_url
}