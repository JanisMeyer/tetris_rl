use rand::{Rng};
const HIDDEN_SIZE : (usize, usize) = (32, 32);

use crate::game;
use game::FEATURE_LENGTH;

#[derive(Clone, Copy)]
pub struct Network {
    w1: [[f64; FEATURE_LENGTH]; HIDDEN_SIZE.0],
    w2: [[f64; HIDDEN_SIZE.0]; HIDDEN_SIZE.1],
    w3: [[f64; HIDDEN_SIZE.1]; 1],
    // intermediate results of the previous forward pass
    previous_hidden1: [f64; HIDDEN_SIZE.0],
    previous_hidden2: [f64; HIDDEN_SIZE.1],
}

// Simple Fully-Connected Neural Network
// w3 * relu ( w2 * relu ( w1 * inp ) )

impl Network {
    pub fn new() -> Network {
        let mut w1 = [[0.0; FEATURE_LENGTH]; HIDDEN_SIZE.0];
        let mut w2 = [[0.0; HIDDEN_SIZE.0]; HIDDEN_SIZE.1];
        let mut w3 = [[0.0; HIDDEN_SIZE.1]; 1];

        // Initialize weights, each in range [-1, 1]
        let mut rand = rand::thread_rng();
        w3[0] = rand.gen();
        for i in 0..HIDDEN_SIZE.1 {
            w3[0][i] = w3[0][i] * 2.0 - 1.0;
            w2[i] = rand.gen();
        }

        for i in 0..HIDDEN_SIZE.0 {
            w1[i] = rand.gen();
            for j in 0..FEATURE_LENGTH {
                w1[i][j] = w1[i][j] * 2.0 - 1.0;
            }
            for j in 0..HIDDEN_SIZE.1 {
                w2[j][i] = w2[j][i] * 2.0 - 1.0;
            }
        }

        Network {
            w1,
            w2,
            w3,
            previous_hidden1: [0.0; HIDDEN_SIZE.0],
            previous_hidden2: [0.0; HIDDEN_SIZE.1]
        }
    }

    // Forward pass, store intermediate results for backward pass
    pub fn forward(&mut self, input: [f64; FEATURE_LENGTH]) -> f64 {
        let mut hidden1 = [0.0; HIDDEN_SIZE.0];
        let mut hidden2 = [0.0; HIDDEN_SIZE.1];

        // First Layer - Linear Part
        for (j, row) in self.w1.iter().enumerate() {
            for i in 0..FEATURE_LENGTH {
                hidden1[j] += input[i] * row[i];
            }
        }
        
        // First Layer - ReLU
        for i in 0..HIDDEN_SIZE.0 {
            if hidden1[i] < 0.0 {
                hidden1[i] = 0.0;
            }
        }

        // Second Layer - Linear part
        for (j, row) in self.w2.iter().enumerate() {
            for i in 0..HIDDEN_SIZE.0 {
                hidden2[j] += hidden1[i] * row[i];
            }
        }

        // Second layer - ReLU 
        for i in 0..HIDDEN_SIZE.1 {
            if hidden2[i] < 0.0 {
                hidden2[i] = 0.0;
            }
        }

        let mut output = 0.0;

        // Output Layer - Linear
        for j in 0..HIDDEN_SIZE.1 {
            output += self.w3[0][j] * hidden2[j];
        }

        self.previous_hidden1 = hidden1;
        self.previous_hidden2 = hidden2;

        output
    }

    // Backward pass, return gradients for each weight
    pub fn backward(&self, error_gradient: f64, features: [f64; FEATURE_LENGTH]) 
            -> ([[f64; FEATURE_LENGTH]; HIDDEN_SIZE.0], [[f64; HIDDEN_SIZE.0]; HIDDEN_SIZE.1], [[f64; HIDDEN_SIZE.1]; 1]) {
        let mut w1_gradient = [[0.0; FEATURE_LENGTH]; HIDDEN_SIZE.0];
        let mut w2_gradient = [[0.0; HIDDEN_SIZE.0]; HIDDEN_SIZE.1];
        let mut w3_gradient = [[0.0; HIDDEN_SIZE.1]; 1];

        // Output layer
        // Gradient w.r.t w3
        for i in 0..HIDDEN_SIZE.1 {
            w3_gradient[0][i] = self.previous_hidden2[i] * error_gradient;
        }

        // Gradient w.r.t layer input
        let mut grad2 = [0.0; HIDDEN_SIZE.1];
        for i in 0..HIDDEN_SIZE.1 {
            grad2[i] += error_gradient * self.w3[0][i];
        }

        // Second Layer - ReLU
        // Gradient w.r.t to ReLU input
        for i in 0..HIDDEN_SIZE.1 {
            if self.previous_hidden2[i] > 0.0 {
                grad2[i] = 1.0;
            } else {
                grad2[i] = 0.0;
            }
        }

        // Second Layer - Linear Part
        // Gradient w.r.t w2
        for j in 0..HIDDEN_SIZE.1 {
            for i in 0..HIDDEN_SIZE.0 {
                w2_gradient[j][i] = grad2[j] * self.previous_hidden1[i];
            }
        }

        // Gradient w.r.t layer input
        let mut grad1 = [0.0; HIDDEN_SIZE.0];
        for i in 0..HIDDEN_SIZE.0 {
            for j in 0..HIDDEN_SIZE.1 {
                grad1[i] += grad2[j] * self.w2[j][i];
            }
        }

        // First Layer - ReLu
        // Gradient w.r.t to ReLU input
        for i in 0..HIDDEN_SIZE.0 {
            if self.previous_hidden1[i] > 0.0 {
                grad1[i] = 1.0
            } else {
                grad1[i] = 0.0
            }
        }

        // First Layer - linear part
        // Gradient w.r.t w1
        for j in 0..HIDDEN_SIZE.0 {
            for i in 0..FEATURE_LENGTH {
                w1_gradient[j][i] = grad1[j] * features[i];
            }
        }

        (w1_gradient, w2_gradient, w3_gradient)
    }

    // perform gradient ascent
    pub fn update_parameters(&mut self, scalar: f64, 
        w1_gradient: [[f64; FEATURE_LENGTH]; HIDDEN_SIZE.0], 
        w2_gradient: [[f64; HIDDEN_SIZE.0]; HIDDEN_SIZE.1], 
        w3_gradient: [[f64; HIDDEN_SIZE.1]; 1]) {
        for j in 0..HIDDEN_SIZE.0 {
            for i in 0..FEATURE_LENGTH {
                self.w1[j][i] += scalar * w1_gradient[j][i];
            }
            for i in 0..HIDDEN_SIZE.1 {
                self.w2[i][j] += scalar * w2_gradient[i][j];
            }
        }
        for i in 0..HIDDEN_SIZE.1 {
            self.w3[0][i] += scalar * w3_gradient[0][i];
        }
    }
}