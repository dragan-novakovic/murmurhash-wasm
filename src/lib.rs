use wasm_bindgen::prelude::*;
use fastmurmur3::murmur3_x64_128;


#[wasm_bindgen]
pub fn hash(key: &str) -> String {
    let bytes = key.as_bytes();
    let hash_result = murmur3_x64_128(bytes, 0);
    return String::from(format!("{:x}", hash_result.to_be()));
}
