use crate::{Board, Player, Move};
use crate::zobrist::ZobristHash;

#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub board: Board,
    pub next_player: Player,
    /// Vec<(next player, Zobrist hash of current state)>
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
        previous_states.push((self.next_player.other(), next_board.hash()));
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

    pub fn does_move_violate_ko(&self, player: Player, the_move: Move) -> bool {
        match the_move {
            Move::Play(point) => {
                let mut next_board = self.board.clone();
                next_board.place_stone(player, &point);
                let next_situation = (player.other(), next_board.hash());
                self.previous_states.contains(&next_situation)
            }
            _ => false
        }
    }

    pub fn is_valid_move(&self, the_move: Move) -> bool {
        match the_move {
            Move::Play(point) => {
                !self.is_over() ||
                    (self.board.get(&point).is_none() &&
                    !self.is_move_self_capture(self.next_player, the_move) &&
                    !self.does_move_violate_ko(self.next_player, the_move))
            }
            _ => true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point;

    #[test]
    fn test_game_is_not_over_after_one_pass() {
        let mut game_state = GameState::new(19);
        game_state = game_state.apply_move(Move::Pass);
        assert!(!game_state.is_over());
    }

    #[test]
    fn test_game_is_over_after_two_consecutive_passes() {
        let mut game_state = GameState::new(19);
        game_state = game_state.apply_move(Move::Pass);
        game_state = game_state.apply_move(Move::Pass);
        assert!(game_state.is_over());
    }

    #[test]
    fn test_move_that_violates_ko_is_recognized() {
        let mut game_state = GameState::new(19);
        game_state = game_state.apply_move(Move::Play(Point::new(1, 2)));  // Black 1,2
        game_state = game_state.apply_move(Move::Play(Point::new(1, 3)));  // White 1,3
        game_state = game_state.apply_move(Move::Play(Point::new(2, 1)));  // Black 2,1
        game_state = game_state.apply_move(Move::Play(Point::new(2, 2)));  // White 2,2
        game_state = game_state.apply_move(Move::Play(Point::new(3, 2)));  // Black 3,2
        game_state = game_state.apply_move(Move::Play(Point::new(3, 3)));  // White 3,3
        game_state = game_state.apply_move(Move::Play(Point::new(10,10))); // Black 10,10
        game_state = game_state.apply_move(Move::Play(Point::new(2, 4)));  // White 2,4
        game_state = game_state.apply_move(Move::Play(Point::new(2, 3)));  // Black 2,3
        assert!(game_state.does_move_violate_ko(Player::White, Move::Play(Point::new(2, 2))));
        assert!(!game_state.does_move_violate_ko(Player::White, Move::Play(Point::new(14, 14))));
    }

}