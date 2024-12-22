use wasm_bindgen::prelude::*;
use chrono::{DateTime, Utc};

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {} ", name));
}