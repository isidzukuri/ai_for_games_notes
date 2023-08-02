use rand::Rng;

#[derive(Debug, Clone)]
pub struct Input {
    pub input_perceptron_idx: usize,
    pub weight: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Perceptron {
    pub inputs_idxs: Vec<usize>,
    pub state: f64, // current
    pub error: f64, //current
    pub threshold_func: fn(f64) -> f64,
}

impl Perceptron {
    pub fn new() -> Self {
        Self {
            inputs_idxs: vec![],
            state: 0.0,
            error: 0.0,
            threshold_func: simple_relu,
        }
    }

    pub fn feed_forward(&mut self, inputs_weights: Vec<f64>, inputs_states: Vec<f64>) {
        // Go throgh each input and sum contribution.
        let mut sum = 0.0;

        for i in 0..self.inputs_idxs.len() {
            sum += inputs_weights[i] * inputs_states[i];
        }

        self.state = self.threshold(sum);
    }

    fn threshold(&self, input: f64) -> f64 {
        (self.threshold_func)(input)
    }
}

#[derive(Debug)]
pub struct NeuralNetwork {
    pub input_perceptrons: Vec<usize>,
    pub hidden_perceptrons: Vec<usize>,
    pub output_perceptrons: Vec<usize>,
    pub perceptron_store: Vec<Perceptron>,
    pub input_store: Vec<Input>,
}

// Usage?
// 1) build nn = NeuralNetwork::build(width, height)
// 2) set input weights
// 3) nn.learn_pattern(inputs, outputs) for each case from training set. Input should be represented as f64 in range [0, 1]
// 4) use nn.generate_output to process input, check output percentron with highest value for result
impl NeuralNetwork {
    pub fn learn_pattern(&mut self, input_states: Vec<f64>, outputs: Vec<f64>) {
        self.generate_output(input_states);
        self.backprop(outputs);
    }

    pub fn generate_output(&mut self, input_states: Vec<f64>) {
        for (i, perceptron_idx) in self.input_perceptrons.iter().enumerate() {
            self.perceptron_store[*perceptron_idx].state = input_states[i];
        }

        for perceptron_idx in self.hidden_perceptrons.iter() {
            self.perceptron_store[*perceptron_idx] = self.feed_forward_perceptron(*perceptron_idx);
        }

        for perceptron_idx in self.output_perceptrons.iter() {
            self.perceptron_store[*perceptron_idx] = self.feed_forward_perceptron(*perceptron_idx);
        }
    }

    fn feed_forward_perceptron(&self, perceptron_idx: usize) -> Perceptron {
        let mut perceptron = self.perceptron_store[perceptron_idx].clone();
        let mut inputs_weights = vec![];
        let mut inputs_states = vec![];

        for inputs_idx in perceptron.inputs_idxs.iter() {
            let input = &self.input_store[*inputs_idx];
            inputs_weights.push(input.weight);
            inputs_states.push(self.perceptron_store[input.input_perceptron_idx].state);
        }

        perceptron.feed_forward(inputs_weights, inputs_states);
        perceptron
    }

    pub fn backprop(&mut self, outputs: Vec<f64>) {
        for (i, perceptron_idx) in self.output_perceptrons.iter().enumerate() {
            let perceptron = self.perceptron_store[*perceptron_idx].clone();
            let error =
                perceptron.state * (1.0 - perceptron.state) * (outputs[i] - perceptron.state);
            let (perceptron, inputs) = self.adjust_weight(perceptron, error);
            for (i, input_idx) in perceptron.inputs_idxs.iter().enumerate() {
                self.input_store[*input_idx] = inputs[i].clone();
            }
            self.perceptron_store[*perceptron_idx] = perceptron;
        }

        for hidden_perceptron_idx in self.hidden_perceptrons.iter() {
            let mut sum = 0.0;

            for output_perceptron_idx in self.output_perceptrons.iter() {
                let output_perceptron = &self.perceptron_store[*output_perceptron_idx];

                let mut weight = 0.0;

                for input_idx in output_perceptron.inputs_idxs.iter() {
                    let input = &self.input_store[*input_idx];
                    if input.input_perceptron_idx == *hidden_perceptron_idx {
                        weight = input.weight;
                    }
                }
                sum += weight * output_perceptron.error;
            }

            let hidden_perceptron = self.perceptron_store[*hidden_perceptron_idx].clone();
            let error = hidden_perceptron.state * (1.0 - hidden_perceptron.state) * sum;

            let (hidden_perceptron, inputs) = self.adjust_weight(hidden_perceptron, error);
            for (i, input_idx) in hidden_perceptron.inputs_idxs.iter().enumerate() {
                self.input_store[*input_idx] = inputs[i].clone();
            }
            self.perceptron_store[*hidden_perceptron_idx] = hidden_perceptron;
        }
    }

    pub fn adjust_weight(
        &self,
        mut perceptron: Perceptron,
        error: f64,
    ) -> (Perceptron, Vec<Input>) {
        let mut inputs = vec![];

        for input_idx in perceptron.inputs_idxs.iter() {
            let mut input = self.input_store[*input_idx].clone();

            let gain = 1.0; // bias?
                            // let gain = 0.8; // bias?
            let state = self.perceptron_store[input.input_perceptron_idx].state;
            let delta_weight = gain * error * state;
            input.weight += delta_weight;
            inputs.push(input);
        }
        perceptron.error = error;
        (perceptron, inputs)
    }

    pub fn build(inputs_len: usize, outputs_len: usize) -> Self {
        let mut nn: NeuralNetwork = NeuralNetwork {
            input_perceptrons: vec![],
            hidden_perceptrons: vec![],
            output_perceptrons: vec![],
            perceptron_store: vec![],
            input_store: vec![],
        };

        nn.build_inputs_layer(inputs_len);
        nn.build_hidden_layer();
        nn.build_output_layer(outputs_len);

        nn
    }

    fn build_inputs_layer(&mut self, inputs_len: usize) {
        for _ in 0..inputs_len {
            let idx = self.perceptron_store.len();
            self.perceptron_store.push(Perceptron::new());
            self.input_perceptrons.push(idx);
        }
    }

    fn build_hidden_layer(&mut self) {
        for i in 0..self.input_perceptrons.len() {
            let mut layer_inputs = vec![];

            for input_idx in self.input_perceptrons.iter() {
                let idx = self.input_store.len();
                self.input_store.push(Input {
                    input_perceptron_idx: *input_idx,
                    weight: rand::thread_rng().gen_range(0.1..0.2),
                });

                layer_inputs.push(idx);
            }

            let idx = self.perceptron_store.len();
            self.perceptron_store.push(Perceptron {
                inputs_idxs: layer_inputs,
                state: 0.0,
                error: 0.0,
                threshold_func: simple_relu,
            });
            self.hidden_perceptrons.push(idx);
        }
    }

    fn build_output_layer(&mut self, outputs_len: usize) {
        for i in 0..outputs_len {
            let mut layer_inputs = vec![];

            for input_idx in self.hidden_perceptrons.iter() {
                let idx = self.input_store.len();
                self.input_store.push(Input {
                    input_perceptron_idx: *input_idx,
                    weight: rand::thread_rng().gen_range(0.1..0.2),
                });

                layer_inputs.push(idx);
            }

            let idx = self.perceptron_store.len();
            self.perceptron_store.push(Perceptron {
                inputs_idxs: layer_inputs,
                state: 0.0,
                error: 0.0,
                threshold_func: simple_relu,
            });
            self.output_perceptrons.push(idx);
        }
    }

    pub fn highest_output_index(&self) -> usize {
      let mut max : usize = 0;

      for (i, idx) in self.output_perceptrons.iter().enumerate() {
        if self.perceptron_store[*idx].state > self.perceptron_store[max].state {
          max = i
        }
      }

      max
    }

    pub fn print(&self) {
          println!("------inputs---------",);
          for idx in self.input_perceptrons.iter() {
              println!("{:?}", self.perceptron_store[*idx]);
          }

          println!("------hidden---------",);
          for idx in self.hidden_perceptrons.iter() {
              println!("{:?}", self.perceptron_store[*idx]);
              for input_idx in self.perceptron_store[*idx].inputs_idxs.iter() {
                  println!("\t{:?}", self.input_store[*input_idx].weight);
              }
          }
          println!("------outputs----------",);
          for idx in self.output_perceptrons.iter() {
              println!("{:?}", self.perceptron_store[*idx]);
              for input_idx in self.perceptron_store[*idx].inputs_idxs.iter() {
                  println!("\t{:?}", self.input_store[*input_idx].weight);
              }
          }
          println!("--------------------");
    }
}

pub fn simple_relu(input: f64) -> f64 {
    input.max(0.0)
}
