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
