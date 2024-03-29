mod utils;

use ram;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn build(script: &str) -> String {
    let tokens = ram::tokenizer::run(script);
    let ast = ram::parser::run(&tokens);
    ram::generator::javascript::gen(&ast)
}
