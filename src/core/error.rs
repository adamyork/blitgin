use wasm_bindgen::prelude::*;
use web_sys::console;

pub struct GameError {}

#[wasm_bindgen]
impl GameError {
    pub fn warn(message: &str) {
        let template: String = "blitgin-js :: WARNING :: ".to_owned();
        let error_message = &String::from(message)[..];
        let full_message = template + error_message;
        console::log_1(&full_message[..].into());
    }
}
