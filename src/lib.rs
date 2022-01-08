use base64::decode;
use image::load_from_memory;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;

// web_sys crate exports JavaScript functions defined by the browser
// wasm_bindgen crate exports functions created / generated by the developer
#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) {
   log(&"Grayscale Called".into()); // into() is used to convert a str into a JavaScript value

   let base64_to_vector = decode(encoded_file).unwrap();
   log(&"Image Decoded".into());

   let img = load_from_memory(&base64_to_vector).unwrap();
   log(&"Image Loaded".into());
}
