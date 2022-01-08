use crate::{Board, Player, Move};
use crate::zobrist::ZobristHash;

#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub board: Board,
    pub next_player: Player,
    /// Vec<(next player, Zobrist hash of current state)>
    previous_states: Vec<(Player, ZobristHash)>,
    moves: Vec<Move>,
}

impl GameState {
    pub fn new(board_size: usize) -> Self {
        Self {
            board: Board::new(board_size),
            next_player: Player::Black,
            previous_states: Vec::new(),
            moves: Vec::new(),
        }
    }

    // For testing
    fn from_board(board: Board, next_player: Player) -> Self {
        Self {
            board,
            next_player,
            previous_states: vec![],
            moves: Vec::new(),
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
        let mut moves = self.moves.clone();
        moves.push(the_move);
        Self {
            board: next_board,
            next_player: self.next_player.other(),
            previous_states,
            moves
        }
    }

    pub fn is_over(&self) -> bool {
        match self.moves.last() {
            None => false,
            Some(the_move) => match the_move {
                Move::Play(_) => false,
                // Over if two consecutive passes
                Move::Pass => match self.moves.get(self.moves.len() - 2) {
                    Some(Move::Pass) => true,
                    _ => false,
                },
                Move::Resign => true
            }
        }
    }

    pub fn is_move_self_capture(&self, player: Player, the_move: Move) -> bool {
        match the_move {
            Move::Play(point) => {
                let mut next_board = self.board.clone();
                next_board.place_stone(player, &point);
                !next_board.is_alive(&point)
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
                !self.is_over() &&
                    self.board.get(&point).is_none() &&
                    !self.is_move_self_capture(self.next_player, the_move) &&
                    !self.does_move_violate_ko(self.next_player, the_move)
            }
            _ => true
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;
    use crate::Point;

    #[test]
    fn test_game_is_not_over_after_one_pass() {
        let mut game_state = GameState::new(19);
        game_state = game_state.apply_move(Move::Play(Point::new(1, 1)));
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

    #[test]
    fn test_self_capture() {
        let board = r#".o.
                       o.o
                       .o."#;
        let board = Board::from_str(board).unwrap();

        let game = GameState::from_board(board, Player::Black);

        assert!(game.is_move_self_capture(Player::Black, Move::Play(Point::new(2, 2))));
    }


    #[test]
    fn test_self_capture_is_not_valid_move() {
        let board = r#".o.
                       o.o
                       .o."#;
        let board = Board::from_str(board).unwrap();

        let game = GameState::from_board(board, Player::Black);

        assert!(!game.is_valid_move(Move::Play(Point::new(2, 2))));
    }

    #[test]
    fn test_move_is_not_self_capture_if_other_group_is_killed() {
        let board = r#"ooo
                       o.o
                       ooo"#;
        let board = Board::from_str(board).unwrap();

        let game = GameState::from_board(board, Player::Black);

        assert!(game.is_valid_move(Move::Play(Point::new(2, 2))));
    }


    #[test]
    fn test_valid_moves() {
        let board = r#".o.o.x.x.
        x.x.o.o.x
        .x.o.o.ox
        x.x.x.xx.
        xx.x.oo.o
        x.o.xx.x.
        .o.oo.o.o
        o.x.xx.x.
        .x.o.oxox"#;
        let board = Board::from_str(board).unwrap();

        let game = GameState::from_board(board, Player::White);

        assert!(game.is_valid_move(Move::Play(Point::new(7, 1))));

        let board = r#".xxxoo.xx
        xxxxxxxx.
        xxxxxxxxx
        .x.xx.oxx
        xxxxxxxxx
        xx.xxxxox
        xxxxoxxox
        xxxxooxoo
        xx.xoooo."#;
        let board = Board::from_str(board).unwrap();

        let game = GameState::from_board(board, Player::Black);

        assert!(game.is_valid_move(Move::Play(Point::new(1, 7))));

    }
}