// pub mod arrive;
pub mod kinematic;
pub mod seek;
pub mod r#static;
pub mod steering_output;
pub mod xyz_coords;

// pub use arrive::*;
pub use kinematic::*;
pub use r#static::*;
pub use seek::*;
pub use steering_output::*;
pub use xyz_coords::*;

pub fn new_orientation(current: f32, velocity: Coord) -> f32 {
    if velocity[0] > 0 {
        // why?
        // https://doc.rust-lang.org/std/primitive.f64.html#method.atan2
        // (velocity[0] as f32).atan2(velocity[1] as f32)
        1.0
    } else {
        current
    }
}
