pub type Coord = [i64; 3];

pub fn sum_coords(a: Coord, b: Coord) -> Coord {
    let mut result: Coord = [0, 0, 0];
    for (i, (aval, bval)) in a.iter().zip(&b).enumerate() {
        result[i] = aval + bval;
    }
    result
}

pub fn minus_coords(a: Coord, b: Coord) -> Coord {
    let mut result: Coord = [0, 0, 0];
    for (i, (aval, bval)) in a.iter().zip(&b).enumerate() {
        result[i] = aval - bval;
    }
    result
}

pub fn multiply_coords(coord: Coord, multiplier: f32) -> Coord {
    let mut result: Coord = [0, 0, 0];
    for (i, val) in coord.iter().enumerate() {
        result[i] = ((*val as f32) * multiplier) as i64;
    }
    result
}
