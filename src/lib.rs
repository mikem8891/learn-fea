mod utils;
mod matrix;
mod global_matrix;

use matrix::*;
use global_matrix::*;

use std::vec;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, learn-fea!");
}


#[wasm_bindgen]
pub fn main() {
    let elasticity = plane_stress_matrix(30000.0, 10000.0, 0.3);
    let nodes: Vec<_> = [
        (0.0, 0.0),
        (1.0, 0.0),
        (0.0, 1.0),
        (1.0, 1.0),
    ].map(|p| Node2D::zero_at(p)).into();
    let element_node_indices = [
        [0, 1, 2],
        [1, 2, 3],
    ];
    let mut model = Fea2DStaticModel::new(&*nodes);
    model.add_elements(&element_node_indices);
    model.create_stiffness_matrix::<3, T3Element>(elasticity);

}

struct Fea2DStaticModel {
    nodes: Box<[Node2D]>,
    /// Node index list of elements
    /// 
    /// Each `Box<[usize]>` in the `nil_of_elements` is a list of node indices
    /// for an element.
    nil_for_elements: Vec<Box<[usize]>>,
    stiffness: Option<GlobalMatrix>,
}

impl Fea2DStaticModel {

    fn new(nodes: &[Node2D]) -> Self {
        Fea2DStaticModel { 
            nodes: Box::from(nodes), 
            nil_for_elements: vec![], 
            stiffness: None,
        }
    }

    fn add_elements<NIL>(&mut self, nil_for_elements: &[NIL])
    where for<'c> &'c NIL: IntoIterator<Item = &'c usize> {
        for nil in nil_for_elements {
            let nil: Box<_> = nil.into_iter().map(|&i| i).collect();
            self.nil_for_elements.push(nil); 
        }
    }

    fn get_element<const N: usize, E: Element2D<N>>(
        &self, indices: &[usize]
    ) -> E {
        let nodes: [Node2D; N] = 
            core::array::from_fn::<_, N, _>(|i| self.nodes[indices[i]]);
        Element2D::new(nodes)
    }

    fn create_stiffness_matrix<const N: usize, E: Element2D<N>>(
        &self, 
        elasticity: Matrix<3, 3>
    ) -> Vec<Vec<f64>> {
        const ZERO_2X2: Matrix<2, 2> = Matrix::<2, 2>::zero();
        let size = 2 * self.nodes.len();
        let mut global_stiffness = vec![vec![Matrix::zero(); size]; size];
        for element_node_indices in self.nil_for_elements.iter() {
            let element: E = self.get_element(&element_node_indices);
            let stiffness_matrices = element.get_stiffness_matrices(elasticity);
            for (i, &node_i) in element_node_indices.iter().enumerate() {
                for (j, &node_j) in element_node_indices.iter().enumerate() {
                    let stiffness_ij = stiffness_matrices[i][j];
                    global_stiffness[node_i][node_j] += &stiffness_ij;
                }
            }
        }
        let global_stiffness = flatten(global_stiffness);
        global_stiffness
    } 

    fn set_stiffness_matrix<const N: usize, E: Element2D<N>>(
        &mut self, 
        elasticity: Matrix<3, 3>
    ) {
        let global_stiffness = self.create_stiffness_matrix::<N, E>(elasticity);
        self.stiffness = Some(GlobalMatrix::new(&global_stiffness));
    }

}

fn get_known_force_indices(nodes: &[Node2D]) -> Vec<usize> {
    let mut indices = vec![];
    for (i, node) in nodes.iter().enumerate() {
        let [known_x, known_y] = &node.known;
        match known_x {
            KnownType::Force => indices.push(2 * i),
            KnownType::Displacement => {},
        }
        match known_y {
            KnownType::Force => indices.push(2 * i + 1),
            KnownType::Displacement => {},
        }
    }
    indices
}

fn flatten<const R: usize, const C: usize>(
    mut block_mat: Vec<Vec<Matrix<R, C>>>
) -> Vec<Vec<f64>> {
    let mut global_mat = vec![vec![]; R];
    for block_i  in 0..block_mat.len() {
        let block_row = &mut block_mat[block_i];
        for block_j in 0..block_row.len() {
            let matrix = block_row[block_j];
            for mat_i in 0..R {
                for mat_j in 0..C {
                    global_mat[R * block_i + mat_i]
                        .push(matrix[(mat_i, mat_j)]);
                }
            }
        }
        *block_row = vec![];
    }
    global_mat
}

#[allow(non_snake_case)]
const fn plane_stress_matrix(E: f64, G: f64, nu: f64) -> Matrix<3,3> {
    let Ep = E / (1.0 - nu * nu);
    let values = [
        [     Ep, Ep * nu, 0.0],
        [Ep * nu,      Ep, 0.0],
        [    0.0,     0.0,   G],
    ];
    Matrix::new(values)
}

type Vector<const D: usize> = Matrix<D, 1>;

type Point2D = Vector<2>;

#[derive(Debug, Clone, Copy)]
enum KnownType {
    Force, 
    Displacement,
}

#[derive(Debug, Clone, Copy)]
struct Node2D {
    position: Point2D,
    displacement: Vector<2>, 
    force: Vector<2>,
    known: [KnownType; 2],
}

impl Node2D {
    fn zero_at(position: (f64, f64)) -> Self {
        let position = position.into();
        let displacement = Matrix::zero();
        let force = Matrix::zero();
        let known = [KnownType::Force, KnownType::Force];
        Node2D {position, displacement, force, known}
    }
}

trait Element2D<const N: usize> {
    fn new(nodes: [Node2D; N]) -> Self;
    fn get_stiffness_matrices(
        &self, 
        elasticity: Matrix<3, 3>
    ) -> [[Matrix<2, 2>; N]; N];
}

struct T3Element {
    nodes: [Node2D; 3],
}

impl T3Element {
    const fn new(nodes: [Node2D; 3]) -> Self {
        T3Element { nodes }
    }

    fn get_trial_functions(&self) -> [T3TrailFunction; 3] {
        [
            T3TrailFunction { element: self, index: 0 },
            T3TrailFunction { element: self, index: 1 },
            T3TrailFunction { element: self, index: 2 },
        ]
    }

    const fn get_node(&self, i: usize) -> &Node2D {
        &self.nodes[i]
    }
    const fn get_position(&self, i: usize) -> &Point2D {
        &self.get_node(i).position
    }

    fn area(&self) -> f64 {
        let n_0 = self.get_position(0);
        let n_1 = self.get_position(1);
        let n_2 = self.get_position(2);
        let n_10 = n_1 - n_0;
        let n_20 = n_2 - n_0;
        n_10.x() * n_20.y() - n_10.y() * n_20.x()
    }
}

impl Element2D<3> for T3Element {
    
    fn new(nodes: [Node2D; 3]) -> Self {
        T3Element { nodes }
    }

    #[allow(non_snake_case)]
    fn get_stiffness_matrices(
        &self, 
        elasticity: Matrix<3,3>
    ) -> [[Matrix<2, 2>; 3]; 3] {
        let mut stiffness_matrices = [[Matrix::zero(); 3]; 3];
        let trial_fns = self.get_trial_functions();
        let trial_grads = trial_fns.map(|tf| tf.gradient().into());
        for (i, &(dNi_dx, dNi_dy)) in trial_grads.iter().enumerate() {
            for (j, &(dNj_dx, dNj_dy)) in trial_grads.iter().enumerate() {
                let diff_i: Matrix<3, 2> = [
                    [dNi_dx,    0.0],
                    [   0.0, dNi_dy],
                    [dNi_dy, dNi_dx],
                ].into();
                let diff_j: Matrix<2, 3> = [
                    [dNj_dx,    0.0, dNj_dy],
                    [   0.0, dNj_dy, dNj_dx],
                ].into();
                stiffness_matrices[i][j] = diff_j * elasticity * diff_i;
            }
        }
        stiffness_matrices
    }
}

struct T3TrailFunction<'a> {
    element: &'a T3Element,
    index: u8, 
}

impl<'a> T3TrailFunction<'a> {

    #[allow(non_snake_case)]
    fn gradient(&self) -> Vector<2> {
        let (x0, y0) = self.element.get_position(0).into();
        let (x1, y1) = self.element.get_position(1).into();
        let (x2, y2) = self.element.get_position(2).into();
        let area = (x1 - x0) * (y2 - y0) - (x2 - x0) * (y1 - y0);
        let dadx = (y2 - y0) / area;
        let dady = (x0 - x2) / area;
        let dbdx = (y0 - y1) / area;
        let dbdy = (x1 - x0) / area;
        assert_ne!(area, 0.0, "nodes are colinear");
        let (dNda, dNdb) = match self.index {
            0 => (-1.0, -1.0),
            1 => ( 1.0,  0.0),
            2 => ( 0.0,  1.0),
            _ => unreachable!(),
        };
        [[dNda * dadx + dNdb * dbdx], [dNda * dady + dNdb * dbdy]].into()
    }
}
