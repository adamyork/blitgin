//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]
extern crate wasm_bindgen_test;
extern crate blitgin;
use wasm_bindgen_test::*;
use blitgin::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    init_hook();
    assert_eq!(true, has_wait_for("hi"));
}