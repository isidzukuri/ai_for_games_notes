use std::collections::HashMap;

use games_ai_book::learning::q_learning::*;

#[test]
fn q_learning_test() {
    let standby_action = Action {
        label: "standby".to_string(),
        reward_func: standby,
    };
    let cooling_action = Action {
        label: "cooling".to_string(),
        reward_func: cooling,
    };
    let heating_action = Action {
        label: "heating".to_string(),
        reward_func: heating,
    };

    let actions = vec![standby_action, cooling_action, heating_action];

    let mut attrs_0: HashMap<String, i32> = HashMap::new();
    attrs_0.insert("hour".to_string(), 0);
    attrs_0.insert("temperature".to_string(), 19);

    let initial_state = State::build(attrs_0, actions.clone());

    let mut problem = Problem {
        states: vec![initial_state],
    };

    let random_state = problem.get_random_state();
    let available_actions = problem.get_available_actions(random_state);

    assert_eq!(*available_actions, actions);

    let (reward, new_state) = problem.take_action(&random_state.actions[2], &random_state);

    if problem.find_state_index(&new_state).is_none() {
        problem.states.push(new_state);
    }

    let trained_model = q_learning(problem, 300, 0.8, 0.85, 0.8, 0.9);

    let mut current_temp = 10;
    let mut current_hour = 0;

    println!("Daily temperature fluctuations:");
    while current_hour < 24 {
        let mut test_attrs_0: HashMap<String, i32> = HashMap::new();
        test_attrs_0.insert("hour".to_string(), current_hour);
        test_attrs_0.insert("temperature".to_string(), current_temp);
        let test_state = State::build(test_attrs_0, vec![]);

        match trained_model.get_best_action(&test_state) {
            Some(best_action) => {
                println!(
                    "hour: {}\ttemp: {}\t[{}]",
                    current_hour, current_temp, best_action.label
                );

                if best_action.label == "cooling".to_string() {
                    current_temp -= 20;
                }
                if best_action.label == "heating".to_string() {
                    current_temp += 20;
                }
                current_temp += get_tempeature_rate(&current_hour).1;
                current_temp = clasterize_temperature(current_temp);
            }
            None => {
                println!("hour: {}\ttemp: {}", current_hour, current_temp);
                current_temp += get_tempeature_rate(&current_hour).1;
                current_temp = clasterize_temperature(current_temp);
            }
        }

        current_hour += 1;
    }
}
// Daily temperature fluctuations:
// hour: 0 temp: -10       [standby]
// hour: 1 temp: -10       [heating]
// hour: 2 temp: -10       [heating]
// hour: 3 temp: -10       [heating]
// hour: 4 temp: -10       [heating]
// hour: 5 temp: -10       [heating]
// hour: 6 temp: -10       [heating]
// hour: 7 temp: -10       [heating]
// hour: 8 temp: 10        [heating]
// hour: 9 temp: 10        [standby]
// hour: 10        temp: 30        [standby]
// hour: 11        temp: 30        [cooling]
// hour: 12        temp: 30        [cooling]
// hour: 13        temp: 30        [cooling]
// hour: 14        temp: 30        [cooling]
// hour: 15        temp: 30        [cooling]
// hour: 16        temp: 30        [cooling]
// hour: 17        temp: 30        [cooling]
// hour: 18        temp: 30        [cooling]
// hour: 19        temp: 30        [cooling]
// hour: 20        temp: 10        [cooling]
// hour: 21        temp: 10        [standby]
// hour: 22        temp: 10        [standby]
// hour: 23        temp: 10        [standby]

#[test]
fn q_value_store_test() {
    let standby_action = Action {
        label: "standby".to_string(),
        reward_func: standby,
    };
    let cooling_action = Action {
        label: "cooling".to_string(),
        reward_func: cooling,
    };
    let heating_action = Action {
        label: "heating".to_string(),
        reward_func: heating,
    };

    let actions = vec![
        standby_action.clone(),
        cooling_action.clone(),
        heating_action.clone(),
    ];

    let mut attrs_0: HashMap<String, i32> = HashMap::new();
    attrs_0.insert("hour".to_string(), 7);
    attrs_0.insert("temperature".to_string(), 19);

    let state_0 = State::build(attrs_0, actions.clone());

    let mut attrs_1: HashMap<String, i32> = HashMap::new();
    attrs_1.insert("hour".to_string(), 8);
    attrs_1.insert("temperature".to_string(), 21);

    let state_1 = State::build(attrs_1, actions.clone());

    let mut qstore = QValueStore { store: vec![] };

    let best_action = qstore.get_best_action(&state_0);
    assert_eq!(best_action, None);

    qstore.store_q_value(&state_0, &standby_action, 5.0);

    assert_eq!(qstore.store.len(), 1);
    assert_eq!(qstore.store[0].0.attrs, state_0.attrs);
    assert_eq!(qstore.store[0].1, standby_action);
    assert_eq!(qstore.store[0].2, 5.0);

    let value = qstore.get_q_value(&state_0, &cooling_action);

    assert_eq!(value, 0.0);

    qstore.store_q_value(&state_0, &standby_action, 5.0);

    assert_eq!(qstore.store.len(), 1);

    qstore.store_q_value(&state_0, &cooling_action, 8.0);

    assert_eq!(qstore.store.len(), 2);

    qstore.store_q_value(&state_1, &cooling_action, 8.0);

    assert_eq!(qstore.store.len(), 3);

    let value = qstore.get_q_value(&state_0, &cooling_action);

    assert_eq!(value, 8.0);

    let best_action = qstore.get_best_action(&state_0).unwrap();

    assert_eq!(best_action, cooling_action);
}
