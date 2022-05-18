extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::env;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent, Button, PressEvent, Key};
use piston::window::{WindowSettings, AdvancedWindow};
use graphics::{clear, Rectangle, Transformed};

const BLOCK_WIDTH : f64 = 30.0;
const BLOCK_HEIGHT : f64 = 30.0;
const LEFT_MARGIN : f64 = 15.0;
const TOP_MARGIN : f64 = 15.0;
const BORDER_WIDTH : f64 = 0.5;

const BLACK : [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE : [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BORDER_COLOR : [f32; 4] = WHITE;

pub mod game;
pub mod agent;
pub mod network;

use game::*;
use agent::Agent;

pub struct App {
    gl: GlGraphics
}

impl App {
    fn render(&mut self, args: &RenderArgs, board: Board, next_piece: Piece) {
        
        
        let square = [1.0, 1.0, BLOCK_WIDTH - 1.0, BLOCK_HEIGHT - 1.0];
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            
            // Game Board
            let transform = c.transform.trans(LEFT_MARGIN - BORDER_WIDTH, TOP_MARGIN - BORDER_WIDTH); 
            Rectangle::new_border(BORDER_COLOR, BORDER_WIDTH)
                .draw(
                    [0.0, 0.0, 
                        RENDER_WIDTH as f64 * BLOCK_WIDTH + BORDER_WIDTH * 2.0, 
                        RENDER_HEIGHT as f64 * BLOCK_HEIGHT + BORDER_WIDTH * 2.0], 
                    &c.draw_state, 
                    transform, 
                    gl);
            // Current state / pieces: simple rectangle for each "pixel"
            for j in 0..RENDER_HEIGHT {
                for i in 0..RENDER_WIDTH {
                    if board.state[j][i] == 0 {
                        continue;
                    }

                    let transform = c.transform
                        .trans(LEFT_MARGIN, TOP_MARGIN)
                        .trans(i as f64 * BLOCK_WIDTH, (RENDER_HEIGHT - 1 - j) as f64 * BLOCK_HEIGHT);

                    let color = PIECE_COLORS[(board.state[j][i] - 1) as usize];
                    Rectangle::new(color).draw(square, &c.draw_state, transform, gl);
                }
            }

            // Draw next shape to the side of the board
            let next_shape = next_piece.shape;
            let transform = c.transform.trans(
                2.0 * LEFT_MARGIN + GAME_WIDTH as f64 * BLOCK_WIDTH,
                TOP_MARGIN + 100.0);
            for j in 0..4 {
                for i in 0..4 {
                    if next_shape[0][j][i] != 0 {
                        let trans = transform.trans(i as f64 * BLOCK_WIDTH, j as f64 * BLOCK_HEIGHT);
                        Rectangle::new(next_piece.color).draw(square, &c.draw_state, trans, gl);
                    }
                }
            }
        });
    }
}

fn init_window_and_app() -> (Window, App) {
    let opengl = OpenGL::V3_2;

    let window: Window = WindowSettings::new(
        "tretris game",
         [3.0 * LEFT_MARGIN + (RENDER_WIDTH + 4) as f64 * BLOCK_WIDTH, 2.0 * TOP_MARGIN + RENDER_HEIGHT as f64 * BLOCK_HEIGHT])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let app = App {
        gl: GlGraphics::new(opengl)
    };

    (window, app)
}

// Run game with input from the user
fn run_user_input(window: &mut Window, app: &mut App) {
    let mut game = Game::new();
    let mut level = game.get_level();
    let mut fall_speed = 0.1 * ((1 - level) as f64 / 3.0).exp();
    window.set_title(format!("Tetris Game - Score: {} - Level: {}", game.score, level));

    let mut time = 0.0;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(window) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Up {
                game.step(SingularAction::Rotate);
            } 
            if key == Key::Left {
                game.step(SingularAction::Left);
            } 
            if key == Key::Right {
                game.step(SingularAction::Right);
            } 
            if key == Key::Down {
                game.step(SingularAction::Down);
            }
        }
        if let Some(args) = e.render_args() {
            app.render(&args, game.state.simulate_board(game.active_piece), game.next_piece);
        }
        if let Some(args) = e.update_args() {
            time += args.dt;
            if time >= fall_speed {
                time = 0.0;
                match game.step(SingularAction::None) {
                    StepResult::GameOver => game = Game::new(),
                    _ => ()
                }
                level = game.get_level();
                fall_speed = 0.1 * ((1 - level) as f64 / 3.0).exp(); // 0.07
                window.set_title(format!("Tetris Game - Score: {} - Level: {}", game.score, level));
            }
        }
    }
}

// run game with AI and training
fn run_agent_input(window: &mut Window, app: &mut App) {
    let mut game = Game::new();
    let mut level = game.get_level();
    let mut fall_speed = 0.1 * ((1 - level) as f64 / 3.0).exp();
    window.set_title(format!("Tetris Game - Score: {} - Level: {}", game.score, level));

    let mut agent = Agent::new();
    let mut action = ComposedAction::new();
    let mut new_piece = true; // wether a new piece was added during the previous time step

    let mut time = 0.0;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(window) {
        if let Some(args) = e.render_args() {
            app.render(&args, game.state.simulate_board(game.active_piece), game.next_piece);
        }
        if let Some(args) = e.update_args() {
            time += args.dt;
            // only get the AI actions when a new piece is introduced
            if new_piece {
                new_piece = false;
                let features = Features::build(game.state); // get the features of the current game state
                let possible_actions = game.get_possible_actions(); // list all posible actions
                action = agent.td_learning(features, possible_actions.0, possible_actions.1, possible_actions.2); // run the update/learning step
            }
            // the sampled action is decomposed and executed over multiple time steps, returning the next to take action and the remaining actions
            let decomposed = action.decompose();
            action = decomposed.1; // remaining actions to be taken in future time steps
            match game.step(decomposed.0) { // action to be taken in this time step
                StepResult::NewPiece => new_piece = true,
                _ => ()
            }

            if time >= fall_speed { // piece drops a single row
                time = 0.0;
                match game.step(SingularAction::None) {
                    StepResult::GameOver => {
                        game = Game::new();
                        action = ComposedAction::new();
                    },
                    StepResult::NewPiece => { // the piece has reached the "ground"
                        new_piece = true;
                    },
                    StepResult::Normal => ()
                }
                level = game.get_level();
                fall_speed = 0.1 * ((1 - level) as f64 / 3.0).exp(); // 0.07
                window.set_title(format!("Tetris Game - Score: {} - Level: {}", game.score, level));
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let window_and_app = init_window_and_app();
    let mut window = window_and_app.0;
    let mut app = window_and_app.1;
    if args.len() > 1 && args[1] == "user" {
        run_user_input(&mut window, &mut app);
    } else {
        run_agent_input(&mut window, &mut app);
    }
}

// TODO: allow rotation even with collision - allow multiple actions at once, smoother Left/Right pressing