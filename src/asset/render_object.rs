use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct RenderObject {
    pub transparency: bool,
    pub show_bounds: bool,
    pub color_constant: Box<str>,
    // rgb_tolerance = {},
    pub workbench: Option<web_sys::HtmlCanvasElement>,
    pub x: i32,
    pub y: i32,
    pub original_x: i32,
    pub original_y: i32,
    pub width: i32,
    pub height: i32,
    pub duration: i32,
    pub frame: i32,
    pub frame_buffer: i32,
    pub velocity_x: i32,
    pub velocity_y: i32,
    pub ease_coefficient: i32,
    pub index: i32,
    pub life_span: i32,
    pub cell_width: i32,
    pub cell_height: i32,
    pub direction: i32,
    pub asset_class: Box<str>,
    // asset = {},
    // asset_data = {},
    // ctx = {},
    // callback = undefined,
    // collision_pixels = [],
    pub keyframe_len: i32,
}

impl RenderObject {
    pub fn init(&mut self) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        self.workbench = Some(document
            .create_element("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()?);
        // @asset = new Image()
        // @assetData = new Image()
        // @asset.onload = @assetLoadComplete.bind this
        // @asset.src = @assetClass
        Ok(())
    }
}

impl Default for RenderObject {
    fn default() -> Self {
        RenderObject {
            transparency: false,
            show_bounds: false,
            color_constant:String::from("default").into_boxed_str(),
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
            asset_class: String::from("default").into_boxed_str(),
            keyframe_len: 0,
        }
    }
}
