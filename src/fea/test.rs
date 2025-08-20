
use crate::math;

use super::*;

pub fn square() {
    let elasticity = math::stack::Matrix::new({
        let E = 30_000.0;
        let nu = 0.3;
        let Ep = E / (1.0 - nu * nu);
        let G  = 10_000.0;
        [
            [     Ep, Ep * nu, 0.0],
            [Ep * nu,      Ep, 0.0],
            [    0.0,     0.0,   G],
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
    let elements = [
        [0, 1, 2],
        [1, 2, 3],
    ].map(|i| T3Element::new(i));
    let mut model = Lin2DStaticModel::new(elasticity);
    model.add_nodes(&nodes);
    *model.known_at(0) = [KnownType::Displacement, KnownType::Displacement];
    *model.known_at(1) = [KnownType::Force, KnownType::Displacement];
    forces.iter().for_each(|&(i, f)| *model.force_at(i) = f.into());
    model.add_elements(&elements);
    model.step_guass_seidel(100);
    let u_0 = *model.displacement_at(0);
    let u_1 = *model.displacement_at(1);
    let u_2 = *model.displacement_at(2);
    let u_3 = *model.displacement_at(3);
    assert_eq!(format!("[{:.5}, {:.5}]", u_0.x(), u_0.y()), "[0.00000, 0.00000]");
    assert_eq!(format!("[{:.5}, {:.5}]", u_1.x(), u_1.y()), "[0.00113, 0.00000]");
    assert_eq!(format!("[{:.5}, {:.5}]", u_2.x(), u_2.y()), "[0.00100, 0.00027]");
    assert_eq!(format!("[{:.5}, {:.5}]", u_3.x(), u_3.y()), "[0.00213, 0.00027]");
    log!("fea::test::square ... passed");
}