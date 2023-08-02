use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Example {
    pub outcome: Outcome,
    pub attributes: HashMap<String, String>, // <key, val>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Outcome {
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct DecisionTreeNode {
    pub test_attribute: Option<String>,
    pub outcome: Option<Outcome>,
    pub branches: HashMap<String, DecisionTreeNode>,
}

impl DecisionTreeNode {
    pub fn new() -> Self {
        Self {
            outcome: None,
            branches: HashMap::new(),
            test_attribute: None,
        }
    }
}

pub fn print_node(node: &DecisionTreeNode, level: usize) {
    let arrow = format!("{}> ", "---".repeat(level));

    println!(
        "{} test_attribute: {:?}, childs: {}",
        arrow,
        node.test_attribute,
        node.branches.len()
    );

    if node.branches.len() > 0 {
        for key in node.branches.keys() {
            println!("{} {}", arrow, key);

            print_node(node.branches.get(key).unwrap(), level + 1);
        }
    } else {
        println!("-{} OUTCOME: {:?}", arrow, node.outcome);
    }
}

pub struct ID3 {}

impl ID3 {
    pub fn call(
        examples: Vec<Example>,
        attributes: Vec<String>,
        mut decision_node: DecisionTreeNode,
    ) -> DecisionTreeNode {
        let initial_entropy = Self::entropy(&examples);

        if initial_entropy <= 0.0 {
            decision_node.outcome = Some(examples[0].outcome.clone());

            return decision_node;
        } else {
            if attributes.len() == 0 {
                println!(
                    "No attributes left for decision to be based on! 
                  Outcome will be assigned based on outcomes proportions of the examples set"
                );

                let outcome_proportions = Self::outcomes_proportions_ordered(&examples);

                decision_node.outcome = Some(outcome_proportions[0].0.clone());

                return decision_node;
            }
        }

        let examples_count = examples.len();

        let mut best_information_gain = 0f64;
        let mut best_split_attribute: Option<&String> = None;
        let mut best_sets: HashMap<String, Vec<Example>> = HashMap::new();

        for attribute in attributes.iter() {
            let sets = Self::split_by_attribute(&examples, attribute);
            let overral_entropy = Self::entropy_of_sets(&sets, examples_count as f64);
            let information_gain = initial_entropy - overral_entropy;

            println!("###### inf gain {} : {}", information_gain, attribute);

            if information_gain > best_information_gain {
                best_information_gain = information_gain;
                best_split_attribute = Some(&attribute);
                best_sets = sets;
            }
        }

        decision_node.test_attribute = best_split_attribute.cloned();

        let mut branches_attributes = vec![];

        for attribute in attributes.iter() {
            if attribute == best_split_attribute.unwrap() {
                continue;
            }

            branches_attributes.push(attribute.clone());
        }

        for key in best_sets.keys() {
            let set = best_sets.get(key).unwrap();

            let child_node = Self::call(
                set.clone(),
                branches_attributes.clone(),
                DecisionTreeNode::new(),
            );

            decision_node.branches.insert(key.clone(), child_node);
        }

        decision_node
    }

    pub fn entropy(examples: &Vec<Example>) -> f64 {
        let examples_count = examples.len() as f64;

        if examples_count == 0.0 {
            return 0.0;
        }

        let mut outcome_tallies: HashMap<String, usize> = HashMap::new();

        for example in examples.iter() {
            *outcome_tallies
                .entry(example.outcome.label.clone())
                .or_insert(0) += 1;
        }

        let outcome_count = outcome_tallies.len();
        if outcome_count == 0 {
            return 0.0;
        }

        let mut entropy = 0f64;

        for outcome_tally in outcome_tallies.values() {
            let proportion = *outcome_tally as f64 / examples_count;
            entropy -= proportion * proportion.log2();
            println!("proportion: {} entropy {}", proportion, entropy);
        }

        entropy
    }

    pub fn outcomes_proportions_ordered(examples: &Vec<Example>) -> Vec<(Outcome, f64)> {
        let mut result = vec![];
        let examples_count = examples.len() as f64;

        let mut outcome_tallies: HashMap<String, usize> = HashMap::new();

        for example in examples.iter() {
            *outcome_tallies
                .entry(example.outcome.label.clone())
                .or_insert(0) += 1;
        }

        for key in outcome_tallies.keys() {
            let val = outcome_tallies.get(key).unwrap();

            let proportion = *val as f64 / examples_count;
            result.push((Outcome { label: key.clone() }, proportion));
        }

        result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        result
    }

    // create set of lists, so we can access each list by attribute value
    pub fn split_by_attribute(
        examples: &Vec<Example>,
        attribute: &String,
    ) -> HashMap<String, Vec<Example>> {
        let mut sets: HashMap<String, Vec<Example>> = HashMap::new();

        for example in examples.iter() {
            sets.entry(example.attributes.get(attribute).unwrap().clone())
                .or_insert(vec![])
                .push(example.clone());
        }

        println!("------------------");
        println!("{:?}", sets);
        println!("------------------");

        sets
    }

    pub fn entropy_of_sets(sets: &HashMap<String, Vec<Example>>, examples_count: f64) -> f64 {
        let mut entropy = 0f64;

        for set in sets.values() {
            let proportion = set.len() as f64 / examples_count;

            entropy -= proportion * Self::entropy(set);
            println!("sets proportion: {} entropy {}", proportion, entropy);
        }

        println!("------------------");
        println!("entropy of sets {}", entropy);
        println!("------------------");
        println!("");
        println!("");

        entropy
    }
}
