use crate::movement::xyz_coords::*;

// chapter 3.1.2
// position - coordinates x,y or x,y,z
// orientation - angle(rad)
// used for not movable objects

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Static {
    pub position: Coord,
    pub orientation: f32,
}
