use crate::movement::xyz_coords::*;
// chapter 3.1.3
// steering behaviors operate with kinematic data. They return accelerations that will change the velocities of a charachter to move around the level

#[derive(Debug, PartialEq, PartialOrd)]
pub struct SteeringOutput {
    pub linear: Coord,
    pub angular: f32,
}
