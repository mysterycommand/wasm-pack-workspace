extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn run() {
    // If the `console_error_panic_hook` feature is enabled this will set a
    // panic hook, otherwise it will do nothing.
    utils::set_panic_hook();
    console::log_1(&JsValue::from_str("run from example-02"));
}
