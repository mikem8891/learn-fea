#![no_main]
#[macro_use]
mod utils;
pub mod math;
pub mod fea;
pub mod fea_output;

use wasm_bindgen::prelude::*;
pub use fea::Lin2DStaticModel;

#[wasm_bindgen]
pub fn main() {
    fea::test::square();
}

#[wasm_bindgen]
pub fn init_fea(e: f64, nu: f64, g: f64) -> Lin2DStaticModel {
    let elasticity = fea::plane_stress_matrix(e, nu, g);
    Lin2DStaticModel::new(elasticity)
}
