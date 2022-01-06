use crate::{Move, game_state::GameState, Point, Player};
use rand::rngs::ThreadRng;
use rand::Rng;

pub trait Agent {
    fn select_move(&mut self, game_state: &GameState) -> Move;
}

pub struct RandomBot {
    rng: ThreadRng,
}

impl Agent for RandomBot {
    fn select_move(&mut self, game_state: &GameState) -> Move {
        // Generate valid candidates
        let mut candidates = Vec::new();
        for row in 1..=game_state.board.rows {
            for col in 1..=game_state.board.cols {
                let candidate = Point::new(row, col);
                if game_state.is_valid_move(Move::Play(candidate)) &&
                    !game_state.board.is_eye(&candidate, game_state.next_player) {
                    candidates.push(candidate);
                }
            }
        }
        // Select randomly among candidates or pass if there are none
        if candidates.is_empty() {
            Move::Pass
        } else {
            let point = candidates[self.rng.gen_range(0..candidates.len())];
            Move::Play(point)
        }
    }
}