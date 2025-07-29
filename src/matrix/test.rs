
use super::*;

#[test]
fn add() {
    let mat_1: Matrix<2, 2> = 
    [
        [1.0, 2.0],
        [3.0, 4.0]
    ].into();
    let mat_2: Matrix<2, 2> = 
    [
        [5.0, 6.0],
        [7.0, 8.0]
    ].into();
    let mat_3 = mat_1 + mat_2;
    assert_eq!(mat_3[(0, 0)], 6.0);
    assert_eq!(mat_3[(0, 1)], 8.0);
    assert_eq!(mat_3[(1, 0)], 10.0);
    assert_eq!(mat_3[(1, 1)], 12.0);
}

#[test]
fn sub() {
    let mat_1: Matrix<2, 2> = 
    [
        [1.0, 2.0],
        [3.0, 4.0]
    ].into();
    let mat_2: Matrix<2, 2> = 
    [
        [5.0, 6.0],
        [7.0, 8.0]
    ].into();
    let mat_3 = mat_1 - mat_2;
    assert_eq!(mat_3[(0, 0)], -4.0);
    assert_eq!(mat_3[(0, 1)], -4.0);
    assert_eq!(mat_3[(1, 0)], -4.0);
    assert_eq!(mat_3[(1, 1)], -4.0);
}

#[test]
fn mul() {
    let mat_1: Matrix<2, 2> = 
    [
        [1.0, 2.0],
        [3.0, 4.0]
    ].into();
    let mat_2: Matrix<2, 2> = 
    [
        [5.0, 6.0],
        [7.0, 8.0]
    ].into();
    let mat_3 = mat_1 * mat_2;
    assert_eq!(mat_3[(0, 0)], 19.0);
    assert_eq!(mat_3[(0, 1)], 22.0);
    assert_eq!(mat_3[(1, 0)], 43.0);
    assert_eq!(mat_3[(1, 1)], 50.0);
}
