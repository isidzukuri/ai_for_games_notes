// use claim::*;
use games_ai_book::movement::{self, *};

#[test]
fn kinematic_upddate() {
    let mut mvmnt = movement::Kinematic {
        position: [1, 2, 3],
        orientation: 2.0,
        velocity: [1, 1, 1],
        rotation: 1.0,
    };

    let streering = movement::SteeringOutput {
        linear: [5, 8, 9],
        angular: 2.0,
    };

    mvmnt.update(streering, 10.0);

    let expected_result = Kinematic {
        position: [11, 12, 13],
        orientation: 12.0,
        velocity: [51, 81, 91],
        rotation: 21.0,
    };

    assert_eq!(mvmnt, expected_result);
}
