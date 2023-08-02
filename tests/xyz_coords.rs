// use claim::*;
use games_ai_book::movement::xyz_coords::*;

#[test]
fn sum_coords_test() {
    let coord_1: Coord = [1, 2, 3];
    let coord_2: Coord = [4, 5, 6];
    let expected_result: Coord = [5, 7, 9];
    let result: Coord = sum_coords(coord_1, coord_2);

    assert_eq!(result, expected_result);
}

#[test]
fn minus_coords_test() {
    let coord_1: Coord = [1, 2, 3];
    let coord_2: Coord = [4, 5, 6];
    let expected_result: Coord = [-3, -3, -3];
    let result: Coord = minus_coords(coord_1, coord_2);

    assert_eq!(result, expected_result);
}

#[test]
fn multiply_coords_test() {
    let coord_1: Coord = [1, 2, 3];
    let result: Coord = multiply_coords(coord_1, 3.0);
    let expected_result: Coord = [3, 6, 9];

    assert_eq!(result, expected_result);
}
