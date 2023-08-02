use std::collections::HashMap;

use games_ai_book::learning::{self};

#[test]
fn id3_test() {
    let outcome_1 = learning::Outcome {
        label: "Attack".to_string(),
    };
    let outcome_2 = learning::Outcome {
        label: "Defend".to_string(),
    };

    let outcome_3 = learning::Outcome {
        label: "Run".to_string(),
    };

    let mut attributes_1: HashMap<String, String> = HashMap::new();
    attributes_1.insert("health".to_string(), "healthy".to_string());
    attributes_1.insert("cover".to_string(), "in_cover".to_string());
    attributes_1.insert("ammo".to_string(), "with_ammo".to_string());
    let example_1 = learning::Example {
        attributes: attributes_1,
        outcome: outcome_1.clone(),
    };

    let mut attributes_2: HashMap<String, String> = HashMap::new();
    attributes_2.insert("health".to_string(), "hurt".to_string());
    attributes_2.insert("cover".to_string(), "in_cover".to_string());
    attributes_2.insert("ammo".to_string(), "with_ammo".to_string());
    let example_2 = learning::Example {
        attributes: attributes_2,
        outcome: outcome_1.clone(),
    };

    let mut attributes_3: HashMap<String, String> = HashMap::new();
    attributes_3.insert("health".to_string(), "healthy".to_string());
    attributes_3.insert("cover".to_string(), "in_cover".to_string());
    attributes_3.insert("ammo".to_string(), "empty".to_string());
    let example_3 = learning::Example {
        attributes: attributes_3,
        outcome: outcome_2.clone(),
    };

    let mut attributes_4: HashMap<String, String> = HashMap::new();
    attributes_4.insert("health".to_string(), "hurt".to_string());
    attributes_4.insert("cover".to_string(), "in_cover".to_string());
    attributes_4.insert("ammo".to_string(), "empty".to_string());
    let example_4 = learning::Example {
        attributes: attributes_4,
        outcome: outcome_2.clone(),
    };

    let mut attributes_5: HashMap<String, String> = HashMap::new();
    attributes_5.insert("health".to_string(), "hurt".to_string());
    attributes_5.insert("cover".to_string(), "exposed".to_string());
    attributes_5.insert("ammo".to_string(), "with_ammo".to_string());
    let example_5 = learning::Example {
        attributes: attributes_5,
        outcome: outcome_2.clone(),
    };

    let mut attributes_6: HashMap<String, String> = HashMap::new();
    attributes_6.insert("health".to_string(), "hurt".to_string());
    attributes_6.insert("cover".to_string(), "exposed".to_string());
    attributes_6.insert("ammo".to_string(), "with_ammo".to_string());
    let example_6 = learning::Example {
        attributes: attributes_6,
        outcome: outcome_3.clone(),
    };

    let mut start_node = learning::DecisionTreeNode::new();

    assert_eq!(
        learning::ID3::entropy(&vec![
            example_1.clone(),
            example_2.clone(),
            example_3.clone(),
            example_4.clone(),
            example_5.clone()
        ]),
        0.9709505944546686
    );

    let set_1 = learning::ID3::split_by_attribute(
        &vec![
            example_1.clone(),
            example_2.clone(),
            example_3.clone(),
            example_4.clone(),
            example_5.clone(),
        ],
        &"health".to_string(),
    );

    assert_eq!(
        set_1
            .get(&"healthy".to_string())
            .unwrap()
            .contains(&example_1),
        true
    );

    assert_eq!(
        set_1
            .get(&"healthy".to_string())
            .unwrap()
            .contains(&example_2),
        false
    );

    assert_eq!(
        set_1
            .get(&"healthy".to_string())
            .unwrap()
            .contains(&example_3),
        true
    );

    assert_eq!(
        set_1
            .get(&"healthy".to_string())
            .unwrap()
            .contains(&example_4),
        false
    );

    assert_eq!(
        set_1
            .get(&"healthy".to_string())
            .unwrap()
            .contains(&example_5),
        false
    );

    assert_eq!(
        set_1.get(&"hurt".to_string()).unwrap().contains(&example_1),
        false
    );

    assert_eq!(
        set_1.get(&"hurt".to_string()).unwrap().contains(&example_2),
        true
    );

    assert_eq!(
        set_1.get(&"hurt".to_string()).unwrap().contains(&example_3),
        false
    );

    assert_eq!(
        set_1.get(&"hurt".to_string()).unwrap().contains(&example_4),
        true
    );

    assert_eq!(
        set_1.get(&"hurt".to_string()).unwrap().contains(&example_5),
        true
    );

    start_node = learning::ID3::call(
        vec![
            example_1,
            example_2,
            example_3,
            example_4,
            example_5.clone(),
            example_5,
            example_6.clone(),
            example_6.clone(),
            example_6,
        ],
        vec![
            "health".to_string(),
            "cover".to_string(),
            "ammo".to_string(),
        ],
        start_node,
    );

    learning::print_node(&start_node, 0);

    assert_eq!(start_node.test_attribute, Some("health".to_string()));
    assert_eq!(
        start_node
            .branches
            .get(&"healthy".to_string())
            .unwrap()
            .test_attribute,
        Some("cover".to_string())
    );
    assert_eq!(
        start_node
            .branches
            .get(&"hurt".to_string())
            .unwrap()
            .test_attribute,
        Some("ammo".to_string())
    );

    assert_eq!(
        start_node
            .branches
            .get(&"hurt".to_string())
            .unwrap()
            .branches
            .get(&"with_ammo".to_string())
            .unwrap()
            .test_attribute,
        Some("cover".to_string())
    );
    assert_eq!(
        start_node
            .branches
            .get(&"hurt".to_string())
            .unwrap()
            .branches
            .get(&"with_ammo".to_string())
            .unwrap()
            .branches
            .get(&"in_cover".to_string())
            .unwrap()
            .test_attribute,
        None
    );
    assert_eq!(
        start_node
            .branches
            .get(&"hurt".to_string())
            .unwrap()
            .branches
            .get(&"with_ammo".to_string())
            .unwrap()
            .branches
            .get(&"in_cover".to_string())
            .unwrap()
            .outcome,
        Some(learning::Outcome {
            label: "Attack".to_string()
        })
    );
    assert_eq!(
        start_node
            .branches
            .get(&"hurt".to_string())
            .unwrap()
            .branches
            .get(&"with_ammo".to_string())
            .unwrap()
            .branches
            .get(&"exposed".to_string())
            .unwrap()
            .outcome,
        Some(learning::Outcome {
            label: "Run".to_string()
        })
    );
}

// learning::print_node(&start_node, 0);
// >  test_attribute: Some("health"), childs: 2
// >  healthy
// --->  test_attribute: Some("cover"), childs: 1
// --->  in_cover
// ------>  test_attribute: Some("ammo"), childs: 2
// ------>  empty
// --------->  test_attribute: None, childs: 0
// ---------->  OUTCOME: Some(Outcome { label: "Defend" })
// ------>  with_ammo
// --------->  test_attribute: None, childs: 0
// ---------->  OUTCOME: Some(Outcome { label: "Attack" })
// >  hurt
// --->  test_attribute: Some("ammo"), childs: 2
// --->  with_ammo
// ------>  test_attribute: Some("cover"), childs: 2
// ------>  exposed
// --------->  test_attribute: None, childs: 0
// ---------->  OUTCOME: Some(Outcome { label: "Run" })
// ------>  in_cover
// --------->  test_attribute: None, childs: 0
// ---------->  OUTCOME: Some(Outcome { label: "Attack" })
// --->  empty
// ------>  test_attribute: None, childs: 0
// ------->  OUTCOME: Some(Outcome { label: "Defend" })
