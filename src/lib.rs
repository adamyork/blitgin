use crate::core::error::GameError;
use crate::core::input::Input;
use wasm_bindgen::prelude::*;
use web_sys::console;
use crate::asset::render_object::RenderObject;

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
    GameError::warn(&"some message");
    if input.has_waits_for(key) == true {
        console::log_1(&"in the if true".into());
        let mut render_obj = RenderObject {
            transparency: false,
            show_bounds: false,
            color_constant:String::from("someValue").into_boxed_str(),
            workbench: None,
            x: 0,
            y: 0,
            original_x: 0,
            original_y: 0,
            width: 0,
            height: 0,
            duration: 0,
            frame: 0,
            frame_buffer: 0,
            velocity_x: 0,
            velocity_y: 0,
            ease_coefficient: 0,
            index: 0,
            life_span: 0,
            cell_width: 0,
            cell_height: 0,
            direction: 0,
            asset_class: String::from("someValue").into_boxed_str(),
            keyframe_len: 0
        };
        let result = render_obj.init();
        if result.is_ok() {
            console::log_1(&"result is true".into());
        }
        return true;
    }
    console::log_1(&"false".into());
    false
}
