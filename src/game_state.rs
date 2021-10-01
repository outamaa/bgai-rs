use crate::{Board, Player, Move};
use crate::zobrist::ZobristHash;

#[derive(Debug, Clone, PartialEq)]
struct GameState {
    board: Board,
    next_player: Player,
    previous_states: Vec<(Player, ZobristHash)>,
    last_move: Option<Move>,
}

impl GameState {
    fn new(board_size: usize) -> Self {
        Self {
            board: Board::new(board_size),
            next_player: Player::Black,
            previous_states: Vec::new(),
            last_move: None
        }
    }

    pub fn apply_move(&self, the_move: Move) -> Self {
        let mut next_board = self.board.clone();
        match the_move {
            Move::Play(point) => {
                next_board.place_stone(self.next_player, &point);
            }
            Move::Pass => {}
            Move::Resign => {}
        }

        let mut previous_states = self.previous_states.clone();
        previous_states.push((self.next_player, next_board.hash()));
        Self {
            board: next_board,
            next_player: self.next_player.other(),
            previous_states,
            last_move: Some(the_move)
        }
    }

    pub fn is_over(&self) -> bool {
        match self.last_move {
            None => false,
            Some(the_move) => match the_move {
                Move::Play(_) => false,
                // Over if two consecutive passes
                Move::Pass => self
                    .previous_states
                    .windows(2)
                    .last()
                    .map(|two_last_states| {
                        // Zobrist hashes of two last states are the same
                        // => Both have passed
                        two_last_states[0].1 == two_last_states[1].1
                    })
                    .unwrap_or(false),
                Move::Resign => true
            }
        }
    }

    pub fn is_move_self_capture(&self, player: Player, the_move: Move) -> bool {
        match the_move {
            Move::Play(point) => {
                let mut next_board = self.board.clone();
                next_board.place_stone(player, &point);
                next_board.is_alive(&point)
            }
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_is_not_over_after_one_pass(){
        let mut game_state = GameState::new(19);
        game_state = game_state.apply_move(Move::Pass);
        assert!(!game_state.is_over());
    }

    #[test]
    fn test_game_is_over_after_two_consecutive_passes(){
        let mut game_state = GameState::new(19);
        game_state = game_state.apply_move(Move::Pass);
        game_state = game_state.apply_move(Move::Pass);
        assert!(game_state.is_over());
    }
}