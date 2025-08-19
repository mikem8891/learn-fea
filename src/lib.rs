#[macro_use]
mod utils;
mod math;
mod fea;

use fea::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    let elasticity = plane_stress_matrix(30000.0, 0.3, 10000.0);
    let nodes = [
        (0.0, 0.0),
        (1.0, 0.0),
        (0.0, 1.0),
        (1.0, 1.0),
    ].map(|p| Node2D::zero_at(p));
    let known_displacements = [0, 1];
    let fixed_node = [KnownType::Displacement, KnownType::Displacement];
    let forces = [
        (2, [15.0, 0.0])
    ].map(|(i, f)| (i, f.into()));
    let elements = [
        [0, 1, 2],
        [1, 2, 3],
    ].map(|i| T3Element::new(i));
    let mut model = Lin2DStaticModel::new(elasticity);
    model.add_nodes(&nodes);
    known_displacements.iter().for_each(|&i| *model.known_at(i) = fixed_node);
    forces.iter().for_each(|&(i, f)| *model.force_at(i) = f);
    model.add_elements(&elements);
    model.step_guass_seidel(100);
    let u3 = *model.displacement_at(2);
    let u4 = *model.displacement_at(3);
    log!(" u_3 = {u3}\n u_4 = {u4}");
}

