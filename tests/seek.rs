use games_ai_book::movement::{self, *};

#[test]
fn seek_get_steering() {
    let charachter = movement::Kinematic {
        position: [1, 2, 3],
        orientation: 2.0,
        velocity: [1, 1, 1],
        rotation: 1.0,
    };

    let target = movement::Kinematic {
        position: [10, 0, 33],
        orientation: 2.0,
        velocity: [1, 1, 1],
        rotation: 1.0,
    };

    let result = movement::Seek {
        charachter: charachter,
        target: target,
        max_acceleration: 3.1,
    }
    .get_steering();

    let expected_result = SteeringOutput {
        linear: [27, -6, 93],
        angular: 0.0,
    };

    assert_eq!(result, expected_result);
}
