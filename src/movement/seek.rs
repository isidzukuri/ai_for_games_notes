use crate::movement::xyz_coords::*;
use crate::movement::Kinematic;
use crate::movement::SteeringOutput;

// chapter 3.3.3

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Seek {
    pub charachter: Kinematic,
    pub target: Kinematic,
    pub max_acceleration: f32,
}

impl Seek {
    pub fn get_steering(self) -> SteeringOutput {
        SteeringOutput {
            linear: multiply_coords(
                minus_coords(self.target.position, self.charachter.position),
                self.max_acceleration,
            ),
            angular: 0.0,
        }
    }
}
