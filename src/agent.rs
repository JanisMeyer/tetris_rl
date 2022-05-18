use rand::{thread_rng, Rng, seq::IteratorRandom};

const BATCH_SIZE : usize = 32;

use crate::game;
use crate::network;

use game::*;
use network::Network;

// Replay Buffer to store game states that can be reused during training
pub struct ReplayBuffer {
    buffer: Vec<Play>
}

impl ReplayBuffer {
    fn new() -> ReplayBuffer {
        ReplayBuffer {
            buffer: Vec::new()
        }
    }

    fn count (&self) -> usize {
        self.buffer.len()
    }

    fn add(&mut self, play: Play) {
        self.buffer.push(play);
    }

    fn sample_batch(&self, batch_size: usize) -> Vec<Play> {
        let mut rng = thread_rng();
        self.buffer.iter().choose_multiple(&mut rng, batch_size).iter().copied().copied().collect()
    }
}

// Agent
pub struct Agent {
    learning_rate : f64,
    discount_factor : f64,
    exploration_rate : f64,
    model: Network,
    target_model: Network,
    replay_buffer: ReplayBuffer,
    current_iteration: i32
}

impl Agent {
    pub fn new() -> Agent {
        let network = Network::new();
        Agent {
            learning_rate: 0.001,
            discount_factor: 0.9,
            exploration_rate: 0.5,
            model: network,
            target_model: network.clone(),
            replay_buffer: ReplayBuffer::new(),
            current_iteration: 0
        }
    }

    // Sample the action using epsilon-greedy exploration strategy
    fn sample_action(&self, action_values: Vec<f64>) -> usize {
        let sample : f64 = rand::thread_rng().gen();
        if sample <= self.exploration_rate { // explore
            rand::thread_rng().gen_range(0, action_values.len())
        } else { // exploit (use best action)
            let mut max_value = action_values[0];
            let mut idx = 0 as usize;
            for i in 1..action_values.len() {
                if action_values[i] > max_value {
                    max_value = action_values[i];
                    idx = i;
                }
            }
            idx
        }
    }

    // LOSS
    fn compute_error(&mut self, play: Play) -> f64 {
        let mut error = play.reward - self.model.forward(play.previous_state.get_vector());
        if !play.next_state.is_terminal {
            error += self.discount_factor * self.target_model.forward(play.next_state.get_vector());
        }
        error
    }

    pub fn td_learning(&mut self, features: Features, possible_actions: Vec<ComposedAction>, rewards: Vec<f64>, sim_features: Vec<Features>) -> ComposedAction {
        // Sample next action, simulate game state
        let mut action_values : Vec<f64> = Vec::new();
        for (i, _) in possible_actions.iter().enumerate() {
            let qsa = rewards[i] + self.discount_factor * self.model.forward(sim_features[i].get_vector());
            action_values.push(qsa);
        }
        let action_idx = self.sample_action(action_values);
        let play = Play {
            previous_state: features,
            next_state: sim_features[action_idx],
            reward: rewards[action_idx],
            action: possible_actions[action_idx]
        };
        self.replay_buffer.add(play);

        // Sample batch of actions/states from replay buffer and use for training
        if self.replay_buffer.count() >= BATCH_SIZE {
            let batch = self.replay_buffer.sample_batch(BATCH_SIZE);
            for sampled_play in batch {
                let error = self.compute_error(sampled_play);
                let gradients = self.model.backward(error, sampled_play.previous_state.get_vector());
                self.model.update_parameters(self.learning_rate * error, gradients.0, gradients.1, gradients.2);
            }

            self.update_hyperparameters();
        }

        play.action
    }

    fn update_hyperparameters(&mut self) {
        if self.exploration_rate > 0.01 {
            self.exploration_rate = self.exploration_rate * 0.99;
        } else {
            self.exploration_rate = 0.01
        }

        if self.learning_rate > 0.00001{
            self.learning_rate = self.learning_rate * 0.999
        } else {
            self.learning_rate = 0.00001;
        }

        // copy network into target network
        self.current_iteration += 1;
        if self.current_iteration % 10 == 0{
            self.target_model = self.model.clone()
        }
    }
}