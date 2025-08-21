#![no_main]
#[macro_use]
mod utils;
pub mod math;
pub mod fea;

use fea::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {

    fea::test::square();
}
