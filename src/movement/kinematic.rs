use crate::movement::xyz_coords::*;
use crate::movement::SteeringOutput;

// chapter 3.1.3
// position - coordinates x,y or x,y,z
// orientation - angle(rad)
// velocity - how fast charachter coords are  changing, x,y or x,y,z
// rotation - how fast charachter orienation is changing

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Kinematic {
    pub position: Coord,
    pub orientation: f32,
    pub velocity: Coord,
    pub rotation: f32,
}

impl Kinematic {
    pub fn update(&mut self, steering: SteeringOutput, time: f32) {
        self.position = sum_coords(self.position, multiply_coords(self.velocity, time));
        self.orientation += self.rotation * time;
        self.velocity = sum_coords(self.velocity, multiply_coords(steering.linear, time));
        self.rotation += steering.angular * time;

        ()
    }
}
