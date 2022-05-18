# Tetris Simulation

Simple tetris game which can be either controlled by the user or by an AI. In the latter case, the AI learns over the course of the training and improves using reinforcement learning. \
The AI sometimes has trouble with the initialization and runs into a dead-end (building a tower on the left side of the board), in which case the game should be re-started to try again with a different initialization.

## Installation

Simply installing rust and running `cargo build` should work.

The game can then be run either with `cargo run` or using the executable generated in `target/debug/`. By default, the program is controlled by the AI (and the AI is trained over time). \
To use user input instead, run with `cargo run user` or start the executable with the argument `user`. The actions can then be controlled with the arrow-keys (up: rotate piece, left/right: move piece to the left/right, down: drop piece)

## Implementation

The code for the game logic can be found in `src/game.rs`. \
`src/network.rs` contains the code for the simple Neural Network, and `src/agent.rs` contains the reinforcement learning agent used for training and playing the game using the AI.

## Reinforcement Learning

The game state is encoded in a simple feature vector of four features (average height of each column, sum of height differences between columns, total number of holes). The agent then uses a simple neural network to evaluate the game state (Q-values: expected future reward after taking a specific action), calculates the temporal difference with the actual future reward (as calculated for the next time step and then estimated using the neural network), and then performs a backward pass on the neural network using the temporal difference as target (loss is calculated as L1-distance). The network weights are then updated using the calculated gradients.

To make the learning more efficient, a Replay Buffer is used as well as a target network, which is periodically copied from the original network, to estimate the possible future reward. This should therefore be equivalent to Q-Learning with Replay Buffers.

The Neural Network is a simple feed-forward neural network with two hidden layers and the ReLU activation function.

## TODO

The next obvious step is to use Double Q-Learning and a different loss function (e.g. Huber Loss), to fix the initialization problems, and use a more complete feature representation of the current game state.