
use crate::math;

use super::*;

pub fn square() {
    let elasticity = math::stack::Matrix::new({
        let e = 30_000.0;
        let nu = 0.3;
        let ep = e / (1.0 - nu * nu);
        let g  = 10_000.0;
        [
            [     ep, ep * nu, 0.0],
            [ep * nu,      ep, 0.0],
            [    0.0,     0.0,   g],
        ]
    });
    let nodes = [
        (0.0, 0.0),
        (1.0, 0.0),
        (0.0, 1.0),
        (1.0, 1.0),
    ].map(|p| Node2D::zero_at(p));
    let forces = [
        (0, [-25.0, -15.0]),
        (1, [ 15.0,  -5.0]),
        (2, [-15.0,   5.0]),
        (3, [ 25.0,  15.0])
    ];
    let mut model = Lin2DStaticModel::new(elasticity);
    model.add_nodes(&nodes);
    let elements = [
        [0, 1, 2],
        [1, 2, 3],
    ];
    let nodes = model.get_nodes();
    let mut nodes = nodes.borrow_mut();
    nodes[0].known = [KnownType::Displacement, KnownType::Displacement];
    nodes[1].known = [KnownType::Force, KnownType::Displacement];
    forces.iter().for_each(|&(i, f)| nodes[i].force = f.into());
    std::mem::drop(nodes);
    model.add_elements(&elements);
    model.step_guass_seidel(100);
    let nodes = model.get_nodes();
    let nodes = nodes.borrow();
    let u_0 = nodes[0].displacement;
    let u_1 = nodes[1].displacement;
    let u_2 = nodes[2].displacement;
    let u_3 = nodes[3].displacement;
    assert_eq!(format!("[{:.5}, {:.5}]", u_0.x(), u_0.y()), "[0.00000, 0.00000]");
    assert_eq!(format!("[{:.5}, {:.5}]", u_1.x(), u_1.y()), "[0.00113, 0.00000]");
    assert_eq!(format!("[{:.5}, {:.5}]", u_2.x(), u_2.y()), "[0.00100, 0.00027]");
    assert_eq!(format!("[{:.5}, {:.5}]", u_3.x(), u_3.y()), "[0.00213, 0.00027]");
    let strain = model.elements[0].get_strain();
    assert_eq!(format!("{:.5}", strain[0]), "0.00113");
    assert_eq!(format!("{:.5}", strain[1]), "0.00027");
    assert_eq!(format!("{:.5}", strain[2]), "0.00100");
    let stress = model.elements[0].get_stress(elasticity);
    assert_eq!(format!("{:.1}", stress[0]), "40.0");
    assert_eq!(format!("{:.1}", stress[1]), "20.0");
    assert_eq!(format!("{:.1}", stress[2]), "10.0");
    log!("fea::test::square ... passed");
}