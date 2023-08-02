use games_ai_book::decision_making::*;

#[test]
fn decision_simple_selector_top_goal_test() {
    let goal1 = Goal {
        name: "g1".to_string(),
        value: 1.0,
    };
    let goal2 = Goal {
        name: "g2".to_string(),
        value: 30.0,
    };
    let goal3 = Goal {
        name: "g3".to_string(),
        value: 5.0,
    };
    let goals = vec![&goal1, &goal2, &goal3];
    let top_goal = SimpleSelecor::top_goal(goals);

    assert_eq!(*top_goal, goal2);
}

// fn decision_simple_selector_choose_action_test() {
//     let goal1 = Goal { name: "g1".to_string(), value: 1.0 };
//     let goal2 = Goal { name: "g2".to_string(), value: 30.0 };
//     let goal3 = Goal { name: "g3".to_string(), value: 5.0 };
//     let goals = vec![&goal1, &goal2, &goal3];
//     let top_goal = SimpleSelecor::choose_action(goals);

//     // assert_eq!(*top_goal, goal2);
// }
