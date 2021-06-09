rm harness/public/blitgin_bg.wasm
rm harness/public/blitgin.js
cargo clean
wasm-pack build --target no-modules --out-dir target/bindgen/output
cp target/bindgen/output/blitgin_bg.wasm harness/public/blitgin_bg.wasm
cp target/bindgen/output/blitgin.js harness/public/blitgin.js
