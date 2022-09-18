pub mod minimax;

use crate::game::GameState;
use rand::rngs::ThreadRng;
use rand::Rng;

pub trait Agent<S: GameState> {
    fn select_move(&mut self, game_state: &S) -> S::Move;
}

pub struct RandomBot {
    rng: ThreadRng,
}

impl RandomBot {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }
}

impl<S: GameState> Agent<S> for RandomBot {
    fn select_move(&mut self, game_state: &S) -> S::Move {
        // Generate valid candidates
        let candidates = game_state.valid_moves();
        // Select randomly among candidates or pass if there are none
        candidates[self.rng.gen_range(0..candidates.len())]
    }
}

pub struct MinimaxBot<S: GameState> {
    plies: u32,
    eval_fn: fn(&S) -> i32,
}

impl<S: GameState> MinimaxBot<S> {
    pub fn new(plies: u32, eval_fn: fn(&S) -> i32) -> Self {
        Self { plies, eval_fn }
    }
}

impl<S: GameState> Agent<S> for MinimaxBot<S> {
    fn select_move(&mut self, game_state: &S) -> S::Move {
        let minimax::OptimalMove { best_move, value }  = minimax::minimax(game_state, self.plies, self.eval_fn);
        println!("Selected {:?} {:?}", best_move, value);
        best_move.expect("Not a valid move")
    }
}

