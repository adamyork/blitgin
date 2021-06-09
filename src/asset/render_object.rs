use log::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlImageElement, Element, Window};

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
        debug!("init called");
        let document: Document = web_sys::window().expect("No Window").document().expect("No Document");
        debug!("document created");
        self.workbench = Some(
            document
                .create_element("canvas")?
                .dyn_into::<web_sys::HtmlCanvasElement>()?,
        );
        debug!("workbench created");
        let result:Result<Element, JsValue> = document.create_element("img");
        if result.is_ok() {
            debug!("image created");
            let next_result = result.unwrap().dyn_into::<web_sys::HtmlImageElement>();
            if next_result.is_ok() {
                debug!("image finished");
            } else {
                let e = next_result.unwrap_err();
                error!("image finished failed {:?}", e);
            }
        } else {
            let e = result.unwrap_err();
            error!("image failed {:?}", e);
        }
        // let image: HtmlImageElement = document
        //     .create_element("image")?
        //     .dyn_into::<web_sys::HtmlImageElement>()?;
        // debug!("image created");
        // image.set_src(&self.asset_class);
        // debug!("image src assigned");
        // let p: Result<::js_sys::Promise, JsValue> = web_sys::window()
        //     .unwrap()
        //     .create_image_bitmap_with_html_image_element(&image);
        // debug!("bitmap created");
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
            color_constant: String::from("default").into_boxed_str(),
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
            asset_class: String::from("test.png").into_boxed_str(),
            keyframe_len: 0,
        }
    }
}
