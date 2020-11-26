mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, testing-wasm!");
}

#[wasm_bindgen(module = "/www/foo.js")]
// same file calling rust: doesn't work
// #[wasm_bindgen(module = "/www/index.js")]
// --> ERROR in ../pkg/snippets/testing-wasm-8ea926e8de57779d/www/index.js
// Module not found: Error: Can't resolve 'testing-wasm' in '/...../testing-wasm/pkg/snippets/testing-wasm-8ea926e8de57779d/www'
//  @ ../pkg/snippets/testing-wasm-8ea926e8de57779d/www/index.js 1:0-37 7:0-13
//  @ ../pkg/testing_wasm_bg.wasm
//  @ ../pkg/testing_wasm.js
//  @ ./index.js
//  @ ./bootstrap.js
extern "C" {
    fn jsfunc();
}

#[wasm_bindgen]
pub fn rustfunc() {
    jsfunc();
}

// #[wasm_bindgen(start)]
// pub fn run() {
//     jsfunc()
// }
