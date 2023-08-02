use crate::movement::xyz_coords::*;
use crate::movement::Kinematic;
use crate::movement::SteeringOutput;

// chapter 3.3.4

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Arrive {
    pub charachter: Kinematic,
    pub target: Kinematic,
    pub max_acceleration: f32,
    pub max_speed: f32,
    pub target_radius: f32,
    pub slow_radius: f32,
    pub time_to_target: f32,
}

impl Arrive {
    // pub fn get_steering(self) -> SteeringOutput {
    //     // let direction = minus_coords(self.target.position, self.charachter.position),
        


    //     // SteeringOutput {
    //     //     linear: multiply_coords(
    //     //         minus_coords(self.target.position, self.charachter.position),
    //     //         self.max_acceleration,
    //     //     ),
    //     //     angular: 0.0,
    //     // }
    // }
}
