use wasm_bindgen::prelude::*;
use fastmurmur3::murmur3_x64_128;


#[wasm_bindgen]
pub fn hash(key: &str) -> String {
    let bytes = key.as_bytes();
    let hash_result = murmur3_x64_128(bytes, 0);
    let mut hex_result = format!("{:x}", hash_result.to_be());

    if hex_result.len() != 32 {
        let mut padding = String::new();
        for _ in 0..(32 - hex_result.len()) {
            padding.push('0');
        }
        hex_result = format!("{}{}", padding, hex_result);
    }

    hex_result
}
