use crate::core::input::Input;
use wasm_bindgen::prelude::*;
use web_sys::console;

pub mod asset;
pub mod core;

#[no_mangle]
#[wasm_bindgen]
pub fn has_wait_for(key: &str) -> bool {
    console::log_1(&"has wait for called".into());
    let mut input = Input::new();
    console::log_1(&"new input created".into());
    input.add_wait_for_action(String::from(key), 32);
    console::log_1(&"wait for action called".into());
    if input.has_waits_for(key) == true {
        console::log_1(&"in the if true".into());
        return true;
    }
    console::log_1(&"false".into());
    false
}
