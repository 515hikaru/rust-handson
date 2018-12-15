extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use sha2::{Digest, Sha256};
use std::collections::hash_map::DefaultHasher;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let data = name.to_string().into_bytes();
    // create a Sha256 object
    let mut default_hasher = DefaultHasher::new();
    let mut hasher = Sha256::new();
    default_hasher.input(data);
    hasher.input(data);
    let result = hasher.result();
    alert(&format!(
        "{}のSHA256のハッシュ値は{:x}です。",
        name, result
    ));
}
