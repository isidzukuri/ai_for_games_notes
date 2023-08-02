use std::collections::HashMap;

use games_ai_book::learning::neural_network::*;

#[test]
fn simple_relu_test() {
    assert_eq!(simple_relu(0.3), 0.3);
    assert_eq!(simple_relu(-0.3), 0.0);
}

#[test]
fn nn_build_test() {
    let nn = NeuralNetwork::build(3, 3);
    assert_eq!(nn.input_perceptrons.len(), 3);
    assert_eq!(nn.hidden_perceptrons.len(), 3);
    assert_eq!(nn.output_perceptrons.len(), 3);
    assert_eq!(nn.perceptron_store.len(), 9);
    assert_eq!(nn.input_store.len(), 18);

    let nn = NeuralNetwork::build(3, 2);
    assert_eq!(nn.input_perceptrons.len(), 3);
    assert_eq!(nn.hidden_perceptrons.len(), 3);
    assert_eq!(nn.output_perceptrons.len(), 2);
    assert_eq!(nn.perceptron_store.len(), 8);
    assert_eq!(nn.input_store.len(), 15);

    let nn = NeuralNetwork::build(8, 3);
    assert_eq!(nn.input_perceptrons.len(), 8);
    assert_eq!(nn.hidden_perceptrons.len(), 8);
    assert_eq!(nn.output_perceptrons.len(), 3);
    assert_eq!(nn.perceptron_store.len(), 19);
    assert_eq!(nn.input_store.len(), 88);
}

fn hour(hour: f64) -> f64 {
    (2f64 * std::f64::consts::PI * hour).sin() / 10.0
}

#[test]
fn nn_learn_test() {
    let mut nn = NeuralNetwork::build(3, 3);

    nn.input_store[0].weight = 0.05;
    nn.input_store[1].weight = 0.3;
    nn.input_store[2].weight = 0.7;

    // // (hour, energy_cost, temperature)
    // // (standby, heating, cooling)

    let inputs = vec![hour(12.0), 0.4, 0.25];
    let outputs = vec![0.9, 0.1, 0.1];
    nn.learn_pattern(inputs, outputs);

    // let inputs = vec![hour(17.0), 0.4, 0.8];
    // let outputs = vec![0.1, 0.1, 0.9];
    // nn.learn_pattern(inputs, outputs);

    // // let inputs = vec![12.0, 40.0, 20.0];
    // // let outputs = vec![0.9, 0.1, 0.1];
    // // nn.learn_pattern(inputs, outputs);

    // // let inputs = vec![12.0, 40.0, 60.0];
    // // let outputs = vec![0.1, 0.1, 0.9];
    // // nn.learn_pattern(inputs, outputs);

    let inputs = vec![hour(12.0), 0.4, 0.25];
    nn.generate_output(inputs);

    nn.print();
    // nn.highest_output_index()
}
