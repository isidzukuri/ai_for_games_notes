use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct State {
    pub actions: Vec<Action>,
    pub attrs: HashMap<String, i32>, // TODO: make attrs polymorphic
}

impl State {
    pub fn new() -> Self {
        Self {
            attrs: HashMap::new(),
            actions: vec![],
        }
    }

    pub fn build(attrs: HashMap<String, i32>, actions: Vec<Action>) -> Self {
        Self {
            attrs: attrs,
            actions: actions,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    pub label: String,
    pub reward_func: fn(&State) -> (State, f64),
}

impl Action {
    pub fn call(&self, state: &State) -> (State, f64) {
        (self.reward_func)(state)
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub states: Vec<State>,
}

impl Problem {
    pub fn get_random_state(&self) -> &State {
        self.states.choose(&mut rand::thread_rng()).unwrap()
    }

    pub fn get_available_actions(&self, state: &State) -> &Vec<Action> {
        let index = self.find_state_index(state).unwrap();

        &self.states[index].actions
    }

    pub fn take_action(&self, action: &Action, state: &State) -> (f64, State) {
        let (new_state, reward) = action.call(state);

        match self.find_state_index(&new_state) {
            None => {
                return (reward, new_state);
            }
            Some(index) => return (reward, self.states[index].clone()),
        }
    }

    pub fn find_state_index(&self, state: &State) -> Option<usize> {
        self.states
            .iter()
            .position(|item| item.attrs == state.attrs)
    }
}

pub struct QValueStore {
    pub store: Vec<(State, Action, f64)>,
}

impl QValueStore {
    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn get_best_action(&self, state: &State) -> Option<Action> {
        let mut options = vec![];

        for entry in self.store.iter() {
            if entry.0.attrs == state.attrs {
                options.push(entry);
            }
        }

        if options.len() == 0 {
            return None;
        }

        Some(
            options
                .iter()
                .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
                .unwrap()
                .1
                .clone(),
        )
    }

    pub fn get_q_value(&self, state: &State, action: &Action) -> f64 {
        // let index = self.find_entry_index(state, action).unwrap();
        match self.find_entry_index(state, action) {
            Some(index) => return self.store[index].2,
            None => return 0.0,
        }
    }

    pub fn store_q_value(&mut self, state: &State, action: &Action, value: f64) {
        match self.find_entry_index(state, action) {
            Some(index) => {
                self.store.remove(index);
            }
            None => {}
        };

        self.store.push((state.clone(), action.clone(), value));
    }

    pub fn print_store(&self) {
        println!("-------Q Store ------");
        println!("size: {}", self.store.len());

        let mut ordered = self.store.clone();
        ordered.sort_by(|a, b| {
            a.0.attrs
                .get(&"hour".to_string())
                .unwrap()
                .partial_cmp(&b.0.attrs.get(&"hour".to_string()).unwrap())
                .unwrap()
        });

        for entry in ordered.iter() {
            println!("{:?} : {} {}", entry.0.attrs, entry.1.label, entry.2);
        }
    }

    fn find_entry_index(&self, state: &State, action: &Action) -> Option<usize> {
        self.store
            .iter()
            .position(|item| item.0.attrs == state.attrs && item.1 == *action)
    }
}

// alpha - the learning rate controls how much influence the current feedback value has over the stored Q-value. Range [0,1]
// value 0 would give algorithm that does not learn

// gamma - the discount rate controls how much an actions Q-value depends on the Q-value at the state it leads to. Range [0, 1]
// A value of 0 would rate every action only in terms of the reward it directly provides.
// The algorithm would learn no long-term strategies involving a sequence of actions.
// A value of 1 would rate the reward for the current action as equally important as the quality of the state it leads to.
// Higher values favor longer sequences of actions< but take correspondingly longer to learn.
// Lower values stabilize faster, but usually support relatively short sequences.

// rho - randomness for exploration controls how often the algorithm will take a random action, rather than the best action it knows so far. Range [0, 1]
// A value 0 will give a pure explotation strategy: the algorithm would exploit its current learning, rinforcing what it already knows.
// A value of 1 will give a pure exploration strategy: the algorithm would always be trying new things, newer benefiting from its existing knowledge.

// nu - the length of walk controls number of iterations that will be carried out in a sequence of connected actions. Range [0, 1]
// A value of 0 would mean the algorithm always uses the state it reached in the previous iteration as the starting state for the next iteration.
// This has a benefit of the algorithm seeing through sequences of actions that might eventually lead to success. It has disadvantage of
// allowing the algorithm yo be cauth in a relativelly small nuer of states from which there is no escape or an escape only
// by a sequence of actions with low Q-values.
// A value of 1  would mean that every iteration starts from a random state. If all states and all actions are equally likely,
// then this is the optimal strategy: it covers widest possible range of states and actions in the smallest possible time.
// In reallity, however, some states and actions are far more prevalent. Some states are as atractors, to which lange number of different action sequencess lead.

pub fn q_learning(
    mut problem: Problem,
    iterations: usize,
    alpha: f64,
    gamma: f64,
    rho: f64,
    nu: f64,
) -> QValueStore {
    let mut qstore = QValueStore { store: vec![] };

    let mut state = problem.get_random_state().clone();

    let mut i = 0;

    while i < iterations {
        if random() < nu {
            let mut state = problem.get_random_state().clone();
        }

        let actions = problem.get_available_actions(&state);

        let action = if random() < rho || qstore.get_best_action(&state).is_none() {
            actions.choose(&mut rand::thread_rng()).unwrap().clone()
        } else {
            qstore.get_best_action(&state).unwrap()
        };

        let (reward, new_state) = problem.take_action(&action, &state);

        if problem.find_state_index(&new_state).is_none() {
            problem.states.push(new_state.clone());
        }

        let q_value = qstore.get_q_value(&state, &action);

        let max_q_value = match qstore.get_best_action(&new_state) {
            Some(best_action) => qstore.get_q_value(&new_state, &best_action),
            None => 0.0,
        };

        let new_q = (1f64 - alpha) * q_value + alpha * (reward + gamma * max_q_value);

        qstore.store_q_value(&state, &action, new_q);

        state = new_state.clone();
        i += 1;
    }

    qstore.print_store();

    qstore
}

pub fn random() -> f64 {
    rand::thread_rng().gen_range(0f64..1.0)
}

// //////////////////////////////// temperature problem /////////////////////////////////////////////
// // qlearning to improve energy consumption levels

const TEMP_STEP: i32 = 10;

pub fn standby(state: &State) -> (State, f64) {
    // println!("=====standby=====");

    let mut new_state = State::build(state.attrs.clone(), state.actions.clone());
    let mut hour = new_state.attrs.get("hour").unwrap().clone();

    if hour < 23 {
        hour += 1;
    } else {
        hour = 0i32;
    }

    *new_state.attrs.get_mut("hour").unwrap() = hour;

    let (system_damage_cost, _, _) = get_maintenance_data(clasterize_temperature(
        *new_state.attrs.get("temperature").unwrap(),
    ));
    let reward = -system_damage_cost * 4;

    (new_state, reward.into())
}

pub fn cooling(state: &State) -> (State, f64) {
    // println!("-----cooling-----");

    let mut new_state = State::build(state.attrs.clone(), state.actions.clone());
    let mut hour = new_state.attrs.get("hour").unwrap().clone();

    if hour < 23 {
        hour += 1;
    } else {
        hour = 0i32;
    }

    let (system_damage_cost, _, cooling_energy) = get_maintenance_data(clasterize_temperature(
        *new_state.attrs.get("temperature").unwrap(),
    ));
    let (energy_cost, env_temperature_change) = get_tempeature_rate(&hour);
    let temperature_change = -TEMP_STEP;

    *new_state.attrs.get_mut("hour").unwrap() = hour;
    *new_state.attrs.get_mut("temperature").unwrap() =
        clasterize_temperature(temperature_change + env_temperature_change);

    let reward =
        -(temperature_change.abs() * energy_cost * cooling_energy / 10 + system_damage_cost * 4);

    (new_state, reward.into())
}

pub fn heating(state: &State) -> (State, f64) {
    // println!("++++++heating++++++");

    let mut new_state = State::build(state.attrs.clone(), state.actions.clone());
    let mut hour = new_state.attrs.get("hour").unwrap().clone();

    if hour < 23 {
        hour += 1;
    } else {
        hour = 0i32;
    }

    let (system_damage_cost, heating_engr, _) = get_maintenance_data(clasterize_temperature(
        *new_state.attrs.get("temperature").unwrap(),
    ));
    let (energy_cost, env_temperature_change) = get_tempeature_rate(&hour);
    let temperature_change = TEMP_STEP;

    *new_state.attrs.get_mut("hour").unwrap() = hour;
    *new_state.attrs.get_mut("temperature").unwrap() =
        clasterize_temperature(temperature_change + env_temperature_change);

    let reward =
        -(temperature_change.abs() * energy_cost * heating_engr / 10 + system_damage_cost * 2);

    (new_state, reward.into())
}

// to decrease number of possible states
pub fn clasterize_temperature(temperature: i32) -> i32 {
    match temperature {
        x if x > 80 => 80,
        x if x < 80 && x >= 40 => 40,
        x if x < 40 && x >= 30 => 30,
        x if x < 30 && x >= 20 => 20,
        x if x < 20 && x >= 10 => 10,
        x if x < 10 && x >= 0 => 0,
        x if x < 0 && x >= -10 => -10,
        x if x < -10 && x >= -20 => -20,
        x if x < -30 => -30,
        _ => 80,
    }
}

pub fn get_tempeature_rate(hour: &i32) -> (i32, i32) {
    let rates = vec![
        // (hour, energy_cost, env_temperature_change)
        (0, 1, -12),
        (1, 1, -12),
        (2, 1, -12),
        (3, 1, -12),
        (4, 1, -12),
        (5, 1, -12),
        (6, 1, -12),
        (7, 1, -12),
        (8, 2, 0),
        (9, 2, 0),
        (10, 2, 21),
        (11, 2, 21),
        (12, 2, 21),
        (13, 2, 21),
        (14, 2, 21),
        (15, 2, 21),
        (16, 2, 21),
        (17, 2, 21),
        (18, 2, 21),
        (19, 2, 21),
        (20, 2, 0),
        (21, 2, 0),
        (22, 2, 0),
        (23, 2, 0),
    ];

    // println!("hhhhhhhhhhh {}", hour);

    let index = rates.iter().position(|item| &item.0 == hour).unwrap();

    // (energy_cost, env_temperature_change)
    (rates[index].1, rates[index].2)
}

pub fn get_maintenance_data(temperature: i32) -> (i32, i32, i32) {
    // temperature   system_damage_cost  heating_engr  cooling_energy
    // 80            100                 1000          60
    // 40            55                  98            50
    // 20            25                  70            10
    // 10            0                   5             0
    // 0             0                   3             0
    // -10           40                  30            20
    // -30           70                  90            40

    let rates = vec![
        (80, 100, 1000, 60),
        (40, 55, 198, 50),
        (30, 45, 180, 30),
        (20, 25, 170, 10),
        (10, 0, 3, 0),
        (0, 0, 3, 0),
        (-10, 45, 20, 20),
        (-20, 50, 30, 30),
        (-30, 70, 90, 60),
    ];

    // let rates = vec![
    //     (79, 100, 1000, 60),
    //     (78, 100, 1000, 60),
    //     (77, 100, 1000, 60),
    //     (76, 100, 1000, 60),
    //     (75, 100, 1000, 60),
    //     (74, 100, 1000, 60),
    //     (73, 100, 1000, 60),
    //     (72, 100, 1000, 60),
    //     (71, 100, 1000, 60),
    //     (70, 100, 1000, 60),
    //     (69, 100, 1000, 60),
    //     (68, 100, 1000, 60),
    //     (67, 100, 1000, 60),
    //     (66, 100, 1000, 60),
    //     (65, 90, 1000, 60),
    //     (64, 80, 1000, 60),
    //     (63, 79, 1000, 60),
    //     (62, 78, 1000, 60),
    //     (61, 77, 1000, 60),
    //     (60, 76, 500, 60),
    //     (59, 60, 500, 60),
    //     (58, 59, 500, 60),
    //     (57, 58, 500, 60),
    //     (56, 58, 500, 60),
    //     (55, 58, 500, 60),
    //     (54, 58, 500, 60),
    //     (53, 58, 500, 60),
    //     (52, 58, 500, 60),
    //     (51, 58, 500, 60),
    //     (50, 58, 500, 60),
    //     (49, 55, 500, 60),
    //     (48, 55, 500, 60),
    //     (47, 55, 500, 60),
    //     (46, 55, 500, 60),
    //     (45, 55, 500, 60),
    //     (44, 55, 200, 60),
    //     (43, 55, 200, 60),
    //     (42, 55, 200, 60),
    //     (41, 55, 200, 60),
    //     (40, 55, 200, 60),
    //     (39, 30, 200, 60),
    //     (38, 30, 200, 60),
    //     (37, 30, 200, 50),
    //     (36, 30, 200, 50),
    //     (35, 30, 200, 50),
    //     (34, 30, 200, 50),
    //     (33, 30, 200, 50),
    //     (32, 30, 200, 50),
    //     (31, 30, 90, 50),
    //     (30, 30, 90, 40),
    //     (29, 28, 80, 40),
    //     (28, 28, 80, 40),
    //     (27, 28, 75, 40),
    //     (26, 28, 75, 40),
    //     (25, 28, 75, 30),
    //     (24, 28, 70, 30),
    //     (23, 28, 70, 30),
    //     (22, 28, 70, 30),
    //     (21, 28, 70, 30),
    //     (20, 25, 70, 20),
    //     (19, 24, 15, 15),
    //     (18, 23, 15, 15),
    //     (17, 15, 15, 15),
    //     (16, 14, 15, 10),
    //     (15, 13, 15, 9),
    //     (14, 12, 15, 8),
    //     (13, 11, 15, 7),
    //     (12, 0, 10, 6),
    //     (11, 0, 10, 3),
    //     (10, 0, 10, 4),
    //     (9, 0, 5, 4),
    //     (8, 0, 5, 1),
    //     (7, 0, 5, 1),
    //     (6, 0, 5, 1),
    //     (5, 0, 5, 1),
    //     (4, 0, 5, 1),
    //     (3, 0, 3, 0),
    //     (2, 0, 3, 0),
    //     (1, 0, 3, 0),
    //     (0, 0, 5, 0),
    //     (-1, 0, 5, 5),
    //     (-2, 0, 5, 5),
    //     (-3, 0, 16, 5),
    //     (-4, 0, 17, 6),
    //     (-5, 0, 18, 8),
    //     (-6, 0, 19, 10),
    //     (-7, 10, 20, 11),
    //     (-8, 10, 20, 13),
    //     (-9, 10, 25, 15),
    //     (-10, 40, 26, 20),
    //     (-11, 41, 27, 30),
    //     (-12, 42, 28, 30),
    //     (-13, 43, 29, 30),
    //     (-14, 44, 30, 30),
    //     (-15, 44, 35, 30),
    //     (-16, 44, 35, 30),
    //     (-17, 44, 35, 40),
    //     (-18, 45, 35, 40),
    //     (-19, 46, 40, 40),
    //     (-20, 47, 40, 40),
    //     (-21, 48, 40, 40),
    //     (-22, 52, 40, 40),
    //     (-23, 53, 50, 50),
    //     (-24, 54, 50, 50),
    //     (-25, 55, 50, 50),
    //     (-26, 65, 50, 50),
    //     (-27, 67, 50, 50),
    //     (-28, 68, 60, 50),
    //     (-29, 69, 70, 50),
    //     (-30, 70, 70, 50),
    // ];

    let index = rates.iter().position(|item| item.0 == temperature).unwrap();

    // (system_damage_cost, heating_engr, cooling_energy)
    (rates[index].1, rates[index].2, rates[index].3)
}
