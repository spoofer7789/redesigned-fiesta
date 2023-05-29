// Fix for now: https://github.com/rustwasm/wasm-bindgen/issues/2774
#![allow(clippy::unused_unit)]

//use wasm_bindgen::prelude::*;

use frontend::app::App;

// Use `std::alloc` as the global allocator.
#[global_allocator]
static ALLOC: std::alloc::System = std::alloc::System;

// This is the entry point for the web app
//#[wasm_bindgen]
 pub fn main() /*-> Result<(), JsValue> */ {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
 //   Ok(())
}
