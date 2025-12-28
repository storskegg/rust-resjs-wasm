use crate::app::log_message;
use web_sys::{self, Document, Window};
// use web_sys::Document;
use wasm_bindgen::prelude::*;

pub mod app;
pub mod aux;
pub mod format;
pub mod lite;
pub mod points;
pub mod render;
pub mod structure;
pub mod syntax;
pub mod web;

#[wasm_bindgen]
pub fn greet() {
    log_message("Hello, console!");
}

#[wasm_bindgen]
pub fn get_document() -> Document {
    let window: Window = web_sys::window().expect("no global `window` exists");
    let d: Document = window.document().expect("should have a document on window");

    d.owner_document().unwrap()
}
