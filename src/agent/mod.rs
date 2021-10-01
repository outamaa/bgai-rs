use crate::{Move, game_state::GameState};

pub trait Agent {
    fn select_move(&self, game_state: &GameState) -> Move  {
        // TODO
        Move::Pass
    }
}