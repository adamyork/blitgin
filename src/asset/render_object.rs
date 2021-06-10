use futures::future::FutureExt;
use futures::TryFutureExt;
use log::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlImageElement, Window, CanvasRenderingContext2d};

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
    pub asset:Option<web_sys::HtmlImageElement>,
    pub asset_data:Option<web_sys::HtmlImageElement>,
    pub ctx: Option<web_sys::CanvasRenderingContext2d>,
    // callback = undefined,
    // collision_pixels = [],
    pub keyframe_len: i32,
}

impl RenderObject {
    pub fn init(&mut self) -> Result<(), JsValue> {
        debug!("init called");
        let document: Document = web_sys::window()
            .expect("No Window")
            .document()
            .expect("No Document");
        debug!("document created");
        self.workbench = Some(
            document
                .create_element("canvas")?
                .dyn_into::<web_sys::HtmlCanvasElement>()?,
        );
        debug!("workbench created");
        // let result: Result<Element, JsValue> = document.create_element("img");
        // if result.is_ok() {
        //     debug!("image created");
        //     let next_result = result.unwrap().dyn_into::<web_sys::HtmlImageElement>();
        //     if next_result.is_ok() {
        //         debug!("image finished");
        //     } else {
        //         let e = next_result.unwrap_err();
        //         error!("image finished failed {:?}", e);
        //     }
        // } else {
        //     let e = result.unwrap_err();
        //     error!("image failed {:?}", e);
        // }
        let image: HtmlImageElement = document
            .create_element("img")?
            .dyn_into::<web_sys::HtmlImageElement>()?;
        debug!("image created");
        image.set_src(&self.asset_class);
        debug!("image src assigned");
        let f = wasm_bindgen_futures::JsFuture::from(
            web_sys::window()
                .unwrap()
                .create_image_bitmap_with_html_image_element(&image)?,
        )
        .map(|x| ());
        wasm_bindgen_futures::spawn_local(f);
        debug!("bitmap created");
        self.ctx = Some(self.workbench
            .to_owned()
            .unwrap()
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap());
        debug!("context retrieved");
        self.keyframe_len = 0;
        if self.transparency || (!self.transparency && self.show_bounds) {
            self.asset_data = self.asset.clone();
        } else {
            //@removeColorConstantAndCache @asset,@assetDat
        }
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
            asset: None,
            asset_data: None,
            ctx: None,
            keyframe_len: 0,
        }
    }
}
