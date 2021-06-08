del harness\public\blitgin_bg.wasm
del harness\public\blitgin.js
cargo clean
cargo build --target wasm32-unknown-unknown --release
wasm-gc target/wasm32-unknown-unknown/release/blitgin.wasm
wasm-bindgen target/wasm32-unknown-unknown/release/blitgin.wasm --target no-modules --out-dir target/bindgen/output
copy target\bindgen\output\blitgin_bg.wasm harness\public\blitgin_bg.wasm
copy target\bindgen\output\blitgin.js harness\public\blitgin.js
