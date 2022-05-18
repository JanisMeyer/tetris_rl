use rand::{thread_rng, seq::SliceRandom};

pub const GAME_WIDTH : usize = 10;
pub const GAME_HEIGHT : usize = 24;

pub const RENDER_WIDTH : usize = GAME_WIDTH;
pub const RENDER_HEIGHT : usize = 20;

// Templates for the pieces / their shape
const S_TEMPLATE : [[[u8; 4]; 4]; 4] = [[[0, 1, 1, 0], [1, 1, 0, 0], [0; 4], [0; 4]],
                                        [[0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 0], [0; 4]],
                                        [[0; 4], [0, 1, 1, 0], [1, 1, 0, 0], [0; 4]],
                                        [[1, 0, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0; 4]]];

const Z_TEMPLATE : [[[u8; 4]; 4]; 4] = [[[2, 2, 0, 0], [0, 2, 2, 0], [0; 4], [0; 4]],
                                        [[0, 0, 2, 0], [0, 2, 2, 0], [0, 2, 0, 0], [0; 4]],
                                        [[0; 4], [2, 2, 0, 0], [0, 2, 2, 0], [0; 4]],
                                        [[0, 2, 0, 0], [2, 2, 0, 0], [2, 0, 0, 0], [0; 4]]];

const J_TEMPLATE : [[[u8; 4]; 4]; 4] = [[[3, 0, 0, 0], [3, 3, 3, 0], [0; 4], [0; 4]],
                                        [[0, 3, 3, 0], [0, 3, 0, 0], [0, 3, 0, 0], [0; 4]],
                                        [[0; 4], [3, 3, 3, 0], [0, 0, 3, 0], [0; 4]],
                                        [[0, 3, 0, 0], [0, 3, 0, 0], [3, 3, 0, 0], [0; 4]]];

const L_TEMPLATE : [[[u8; 4]; 4]; 4] = [[[0, 0, 4, 0], [4, 4, 4, 0], [0; 4], [0; 4]],
                                        [[0, 4, 0, 0], [0, 4, 0, 0], [0, 4, 4, 0], [0; 4]],
                                        [[0; 4], [4, 4, 4, 0], [4, 0, 0, 0], [0; 4]],
                                        [[4, 4, 0, 0], [0, 4, 0, 0], [0, 4, 0, 0], [0; 4]]];

const I_TEMPLATE : [[[u8; 4]; 4]; 4] = [[[0; 4], [5, 5, 5, 5], [0; 4], [0; 4]],
                                        [[0, 0, 5, 0], [0, 0, 5, 0], [0, 0, 5, 0], [0, 0, 5, 0]],
                                        [[0; 4], [0; 4], [5, 5, 5, 5], [0; 4]],
                                        [[0, 5, 0, 0], [0, 5, 0, 0], [0, 5, 0, 0], [0, 5, 0, 0]]];

const O_TEMPLATE : [[[u8; 4]; 4]; 4] = [[[0, 6, 6, 0], [0, 6, 6, 0], [0; 4], [0; 4]]; 4];

const T_TEMPLATE : [[[u8; 4]; 4]; 4] = [[[0, 7, 0, 0], [7, 7, 7, 0], [0; 4], [0; 4]],
                                        [[0, 7, 0, 0], [0, 7, 7, 0], [0, 7, 0, 0], [0; 4]],
                                        [[0; 4], [7, 7, 7, 0], [0, 7, 0, 0], [0; 4]],
                                        [[0, 7, 0, 0], [7, 7, 0, 0], [0, 7, 0, 0], [0; 4]]];

const PIECE_TEMPLATES : [[[[u8; 4]; 4]; 4]; 7] = [S_TEMPLATE, Z_TEMPLATE, J_TEMPLATE, L_TEMPLATE, I_TEMPLATE, O_TEMPLATE, T_TEMPLATE];

const RED : [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN : [f32; 4] = [0.0, 0.5, 0.0, 1.0];
const BLUE : [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const YELLOW : [f32; 4] = [1.0, 1.0, 0.0, 1.0];
const CYAN : [f32; 4] = [0.0, 1.0, 1.0, 1.0];
const ORANGE : [f32; 4] = [1.0, 0.64, 0.0, 1.0];
const PURPLE : [f32; 4] = [0.54, 0.16, 0.88, 1.0];
pub const PIECE_COLORS : [[f32; 4]; 7] = [GREEN, RED, BLUE, ORANGE, CYAN, YELLOW, PURPLE];

pub const FEATURE_LENGTH : usize = 4; //GAME_WIDTH * 2 + 2;

// Piece Generator: all 7 pieces are shuffled into a random order instead of simple picking a new piece every time by random
#[derive(Clone, Copy)]
struct PieceGenerator {
    bag : [u8; 7],
    idx : usize
}

impl PieceGenerator {
    fn new() -> PieceGenerator {
        let mut rng = thread_rng();
        let mut bag = [1, 2, 3, 4, 5, 6, 7];
        bag.shuffle(&mut rng);

        PieceGenerator {
            bag,
            idx: 0
        }
    }

    fn get_next(&mut self) -> Piece {
        let mut rng = thread_rng();
        let piece = Piece::new_from_idx(self.bag[self.idx]);
        self.idx += 1;
        if self.idx > 6 {
            self.idx = 0;
            self.bag.shuffle(&mut rng);
        }
        piece
    }
}

// Piece
#[derive(Clone, Copy)]
pub struct Piece {
    pub shape: [[[u8; 4]; 4]; 4],
    position: (i32, i32),
    rotation: i32,
    num_rotations: i32,
    pub color: [f32; 4]
}

impl Piece {
    fn new_from_idx(piece: u8) -> Piece {
        Piece {
            shape: PIECE_TEMPLATES[(piece - 1) as usize],
            position: (3, GAME_HEIGHT as i32 - 1),
            rotation: 0,
            num_rotations: if piece == 6 {1} else {4},
            color: PIECE_COLORS[(piece - 1) as usize]
        }
    }

    fn simulate_next(&self, action: SingularAction) -> Piece {
        let mut piece = self.clone();
        match action {
            SingularAction::Left => { 
                piece.position.0 -= 1;
            },
            SingularAction::Right => {
                piece.position.0 += 1;
            },
            SingularAction::Down => {
                piece.position.1 -= 1;
            },
            SingularAction::Rotate => {
                piece.rotation = (piece.rotation + 1) % 4;
            }
            _ => ()
        }
        piece
    }

    fn simulate_action(&self, action: ComposedAction) -> Piece {
        let mut piece = self.clone();
        piece.position.0 += action.shift;
        piece.rotation = action.rotation;

        piece
    }

    fn get_aabb(&self, rotation: usize) -> [(i32, i32);2] {
        let shape = self.shape[rotation];
        let mut minmax = [(-1 as i32, -1 as i32); 2];
        for (j, row) in shape.iter().enumerate() {
            for (i, value) in row.iter().enumerate() {
                if *value != 0 {
                    if minmax[0].1 == -1 {
                        minmax[0].1 = j as i32;
                    }
                    minmax[1].1 = j as i32;

                    if minmax[0].0 == -1 {
                        minmax[0].0 = i as i32;
                    }
                    minmax[1].0 = i as i32;
                }
            }
        }

        [(minmax[0].0, minmax[0].1), (minmax[1].0 - minmax[0].0, minmax[1].1 - minmax[0].1)]
    }
}

// Board
#[derive(Clone, Copy)]
pub struct Board {
    pub state: [[u8; GAME_WIDTH]; GAME_HEIGHT]
}
fn is_row_full(row: &[u8; GAME_WIDTH]) -> bool {
    for value in row.iter() {
        if *value == 0 {
            return false;
        }
    }
    true
}

impl Board {
    fn remove_full_rows(&self) -> (Board, i32) {
        let mut removed_rows = 0;
        let mut state = self.state.clone();
        for (j, row) in self.state.iter().enumerate().rev() {
            if is_row_full(row) {
                for _j in j..GAME_HEIGHT - 1 {
                    state[_j] = state[_j + 1];
                }
                removed_rows += 1
            }

        }
        (Board {
            state
        }, removed_rows)
    }

    pub fn simulate_board(&self, piece: Piece) -> Board {
        self.integrate_piece(piece)
    }

    fn integrate_piece(&self, piece: Piece) -> Board {
        self.fill_rect(piece.position, piece.shape[piece.rotation as usize])
    }

    fn is_valid_state(&self, piece: Piece) -> bool {
        let offset_x = piece.position.0;
        let offset_y = piece.position.1;
        for (j, row) in piece.shape[piece.rotation as usize].iter().enumerate() {
            for(i, value) in row.iter().enumerate() {
                if *value != 0 {
                    let x = i as i32 + offset_x;
                    let y = offset_y - j as i32;
                    if x < 0 || x as usize >= GAME_WIDTH {
                        return false;
                    }
                    if y < 0 || y as usize >= GAME_HEIGHT {
                        return false;
                    }

                    if self.state[y as usize][x as usize] != 0 {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn fill_rect(&self, offset: (i32, i32), content: [[u8; 4]; 4]) -> Board {
        let mut state = self.state.clone();
        for (j, row) in content.iter().enumerate() {
            for (i, value) in row.iter().enumerate() {
                if *value != 0 {
                    state[(offset.1 - j as i32 ) as usize][(i as i32 + offset.0) as usize] = *value;
                }
            }
        }
        Board {
            state
        }
    }
}

#[derive(Clone, Copy)]
pub enum SingularAction {
    None,
    Left,
    Right,
    Rotate,
    Down
}

// Action of multiple Singular Actions performed in sequence (over multiple time steps)
#[derive(Clone, Copy)]
pub struct ComposedAction {
    pub rotation : i32,
    pub shift: i32
}

impl ComposedAction {
    pub fn new() -> ComposedAction {
        ComposedAction {
            rotation: 0,
            shift: 0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.rotation == 0 && self.shift == 0
    }

    // Return the next singular action and the remaining actions
    pub fn decompose(&self) -> (SingularAction, ComposedAction) {
        let mut action = SingularAction::None;
        let mut remaining_action = self.clone();

        if remaining_action.rotation != 0 {
            action = SingularAction::Rotate;
            remaining_action.rotation -= 1;
        } else {
            if remaining_action.shift < 0 {
                action = SingularAction::Left;
                remaining_action.shift += 1;
            } else if remaining_action.shift > 0 {
                action = SingularAction::Right;
                remaining_action.shift -= 1;
            }
        }
        (action, remaining_action)
    }
}

#[derive(Clone, Copy)]
pub struct Features {
    heights: [i32; GAME_WIDTH],
    height_differences: [i32; GAME_WIDTH - 1],
    maximum_height: i32,
    average_height: f64,
    number_of_holes: [i32; GAME_WIDTH],
    total_no_holes: i32,
    pub is_terminal: bool,
    sum_of_height_differences: i32
}

impl Features {
    // construct features from the current board state
    pub fn build(board: Board) -> Features {
        let mut heights = [0; GAME_WIDTH];
        let mut height_differences = [0; GAME_WIDTH - 1];
        let mut maximum_height = 0;
        let mut average_height = 0.0;
        let mut number_of_holes = [0; GAME_WIDTH];
        let mut total_no_holes = 0;
        let mut sum_of_height_differences = 0;

        for i in 0..GAME_WIDTH {
            for j in 0..GAME_HEIGHT {
                if board.state[j][i] != 0 {
                    heights[i] = j as i32 + 1;
                }
            }
            let mut occupied = false;
            for j in (0..GAME_HEIGHT).rev() {
                if board.state[j][i] != 0 {
                    occupied = true;
                } else if occupied {
                    number_of_holes[i] += 1;
                    total_no_holes += 1;
                }
            }

            average_height += heights[i] as f64;
            if heights[i] > maximum_height {
                maximum_height = heights[i];
            }

            if i > 0 {
                height_differences[i - 1] = (heights[i] - heights[i - 1]).abs();
                sum_of_height_differences += height_differences[i - 1];
            }
        }

        average_height = average_height / GAME_WIDTH as f64;

        Features {
            heights, 
            height_differences,
            maximum_height, 
            average_height,
            number_of_holes,
            total_no_holes,
            is_terminal: false,
            sum_of_height_differences
        }
    }

    // return the feature vector
    pub fn get_vector(&self) -> [f64; FEATURE_LENGTH] {
        let mut feature_vector = [0.0; FEATURE_LENGTH];

        // for i in 0..GAME_WIDTH {
        //     feature_vector[i] = self.heights[i] as f64;
        // }

        // for i in GAME_WIDTH..2*GAME_WIDTH - 1 {
        //     feature_vector[i] = self.height_differences[i - GAME_WIDTH] as f64;
        // }

        // feature_vector[2 * GAME_WIDTH - 1] = self.maximum_height as f64;
        // feature_vector[2 * GAME_WIDTH] = self.total_no_holes as f64;
        // feature_vector[2 * GAME_WIDTH + 1] = 1.0;
        feature_vector[0] = self.average_height;
        feature_vector[1] = self.sum_of_height_differences as f64;
        feature_vector[2] = self.total_no_holes as f64;
        feature_vector[3] = 1.0;
        feature_vector

    }
}

pub enum StepResult {
    Normal,
    NewPiece,
    GameOver
}

// Game
pub struct Game {
    pub state: Board,

    pub active_piece : Piece,
    pub next_piece : Piece,
    pub score : i32,

    generator : PieceGenerator
}

impl Game {
    pub fn new() -> Game {
        let mut generator = PieceGenerator::new();
        let active_piece = generator.get_next();
        let next_piece = generator.get_next();
        Game {
            state: Board {
                state: [[0; GAME_WIDTH]; GAME_HEIGHT]
            },
            active_piece,
            next_piece,
            score: 0,
            generator
        }
    }

    pub fn get_level(&self) -> i32 {
        self.score / 10 + 1
    }

    pub fn get_possible_actions(&self) -> (Vec<ComposedAction>, Vec<f64>, Vec<Features>) {
        let features = Features::build(self.state);

        let mut possible_actions : Vec<ComposedAction> = Vec::new();
        let mut rewards : Vec<f64> = Vec::new();
        let mut v_sim_features : Vec<Features> = Vec::new();

        for rotation in 0..self.active_piece.num_rotations {
            let aabb = self.active_piece.get_aabb(rotation as usize);
            for shift in (-self.active_piece.position.0 - aabb[0].0)..(GAME_WIDTH as i32 - (self.active_piece.position.0 + aabb[1].0)) {
                let action = ComposedAction{ rotation, shift: shift as i32};
                match self.simulate_action(action, features.average_height) {
                    Some((reward, sim_features)) => {
                        rewards.push(reward);
                        v_sim_features.push(sim_features);
                        possible_actions.push(action);
                    },
                    None => continue
                }
            }
        }
        (possible_actions, rewards, v_sim_features)
    }

    // return a game state after the action has been taken
    pub fn simulate_action(&self, action: ComposedAction, avg_height: f64) -> Option<(f64, Features)> {
        let mut piece = self.active_piece.simulate_action(action);
        if !self.state.is_valid_state(piece) {
            return None;
        }

        let mut piece_next = piece.simulate_next(SingularAction::Down);
        while self.state.is_valid_state(piece_next) {
            piece = piece_next.clone();
            piece_next = piece_next.simulate_next(SingularAction::Down);
        }
        let mut simulated_board = self.state.integrate_piece(piece);
        let board_update = simulated_board.remove_full_rows();
        simulated_board = board_update.0;
        let mut reward = board_update.1 as f64;
        let mut features = Features::build(simulated_board);
        reward = reward + avg_height - features.average_height;
        if !simulated_board.is_valid_state(self.next_piece){
            features.is_terminal = true;
            reward = -5.0;
        }
        Some((reward, features))
    }

    pub fn step(&mut self, action : SingularAction) -> StepResult {
        match action {
            SingularAction::None => {
                let piece = self.active_piece.simulate_next(SingularAction::Down);
                if self.state.is_valid_state(piece) {
                    self.active_piece = piece;
                } else {
                    self.state = self.state.integrate_piece(self.active_piece);
                    let board_update = self.state.remove_full_rows();
                    self.state = board_update.0;
                    self.score += board_update.1 + (board_update.1 % 4) * 10;
                    if !self.state.is_valid_state(self.next_piece){
                        return StepResult::GameOver;
                    }
                    self.active_piece = self.next_piece;
                    self.next_piece = self.generator.get_next();
                    return StepResult::NewPiece;
                }
            },
            SingularAction::Left => {
                let piece = self.active_piece.simulate_next(action);
                if self.state.is_valid_state(piece) {
                    self.active_piece = piece;
                }
            },
            SingularAction::Right => {
                let piece = self.active_piece.simulate_next(action);
                if self.state.is_valid_state(piece) {
                    self.active_piece = piece;
                }
            },
            SingularAction::Down => {
                let mut piece = self.active_piece.simulate_next(action);
                while self.state.is_valid_state(piece) {
                    self.active_piece = piece;
                    piece = piece.simulate_next(action);
                }
                self.state = self.state.integrate_piece(self.active_piece);
                let board_update = self.state.remove_full_rows();
                self.state = board_update.0;
                self.score += board_update.1 + (board_update.1 / 4) * 10;
                if !self.state.is_valid_state(self.next_piece){
                    return StepResult::GameOver;
                }
                self.active_piece = self.next_piece;
                self.next_piece = self.generator.get_next();
                return StepResult::NewPiece;
            }, 
            SingularAction::Rotate => {
                let piece = self.active_piece.simulate_next(action);
                if self.state.is_valid_state(piece) {
                    self.active_piece = piece;
                }
            }
        }
        StepResult::Normal
    }
}

#[derive(Clone, Copy)]
pub struct Play {
    pub previous_state: Features,
    pub next_state: Features,
    pub reward: f64,
    pub action: ComposedAction
}