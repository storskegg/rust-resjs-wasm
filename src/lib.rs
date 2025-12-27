use crate::app::log_message;
use wasm_bindgen::prelude::*;

pub mod app;

#[wasm_bindgen]
pub fn greet() {
    log_message("Hello, console!");
}

