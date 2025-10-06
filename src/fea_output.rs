use std::cell::RefCell;

use wasm_bindgen::prelude::*;

// v - e + f = 1

// each new triangular element needs
// 1) 1 vert and 2 edges, or
// 2) 1 edge 

// f = - v + e - 1

pub const FACES: usize = 32;

thread_local! {
    pub static VERTICES: RefCell<[f32; 3 * 2 * FACES]> = RefCell::new([0.0; 3 * 2 * FACES]);
    pub static DISPLACEMENTS: RefCell<[f32; 3 * 2 * FACES]> = RefCell::new([0.0; 3 * 2 * FACES]);
    pub static STRESSES: RefCell<[f32; 3 * 3 * FACES]> = RefCell::new([0.0; 3 * 3 * FACES]);
    pub static FORCES: RefCell<[f32; 3 * 2 * FACES]> = RefCell::new([0.0; 3 * 2 * FACES]);
}

#[wasm_bindgen]
pub fn get_vertices() -> *const f32 {
    VERTICES.with_borrow(|v| v.as_ptr())
}

#[wasm_bindgen]
pub fn get_displacements() -> *const f32 {
    DISPLACEMENTS.with_borrow(|d| d.as_ptr())
}

#[wasm_bindgen]
pub fn get_stresses() -> *const f32 {
    STRESSES.with_borrow(|s| s.as_ptr())
}

#[wasm_bindgen]
pub fn get_forces() -> *const f32 {
    FORCES.with_borrow(|f| f.as_ptr())
}
