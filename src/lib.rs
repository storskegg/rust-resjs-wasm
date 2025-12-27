use crate::app::log_message;
use web_sys::{Document};
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

pub fn get_document() -> &'static Document {
    let window = web_sys::window().expect("no global `window` exists");
    &window.document().expect("should have a document on window")
}
