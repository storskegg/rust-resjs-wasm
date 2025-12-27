use wasm_bindgen::prelude::*;
use web_sys;

pub mod aux;
pub mod format;
pub mod lite;
pub mod points;
pub mod render;
pub mod structure;
pub mod syntax;
pub mod web;

/// Log a message using the active logger (native or WASM).
pub fn log_message(message: &str) {
    {
        web_sys::console::log_1(&format!("WASM logger: {}", message).into());
    }
}




#[wasm_bindgen]
pub fn greet() {
    log_message("Hello, console!");
}

#[wasm_bindgen]
pub fn get_document() -> web_sys::Document {
    let window: web_sys::Window = web_sys::window().expect("no global `window` exists");
    let d: web_sys::Document = window.document().expect("should have a document on window");

    d.owner_document().unwrap()
}
