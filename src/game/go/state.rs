use crate::game::GameState;
use crate::game::go::board::{Board, EmptyBoardPoints};
use crate::game::go::player::Player;
use crate::game::go::types::{Color, Move};
use crate::game::go::zobrist::ZobristHash;

#[derive(Debug, Clone, PartialEq)]
pub struct GoState {
    pub board: Board,
    pub next_player: Player,
    pub previous_player: Player,
    // bit of a code smell here, but ...
    /// Vec<(next player, Zobrist hash of current state)>
    previous_states: Vec<(Color, ZobristHash)>,
    pub moves: Vec<Move>,
}

impl GoState {
    pub fn new(board_size: usize) -> Self {
        Self::from_board(Board::new(board_size), Player::black())
    }

    // For testing
    fn from_board(board: Board, next_player: Player) -> Self {
        let other_color = next_player.color.other();
        Self {
            board,
            next_player,
            previous_player: Player::new(other_color),
            previous_states: Vec::new(),
            moves: Vec::new(),
        }
    }

    pub fn is_move_self_capture(&self, color: Color, the_move: &Move) -> bool {
        match *the_move {
            Move::Play(point) => {
                let mut next_board = self.board.clone();
                next_board.place_stone(color, &point).unwrap();
                !next_board.is_alive(&point)
            }
            _ => false
        }
    }

    pub fn does_move_violate_ko(&self, color: Color, the_move: &Move) -> bool {
        match *the_move {
            Move::Play(point) => {
                let mut next_board = self.board.clone();
                next_board.place_stone(color, &point).expect("Illegal placement");
                let next_situation = (color.other(), next_board.hash());
                self.previous_states.contains(&next_situation)
            }
            _ => false
        }
    }
}

impl GameState for GoState {
    type Move = Move;

    fn apply_move(&self, m: &Self::Move) -> Self {
        let mut next_board = self.board.clone();
        let mut captured_stones = 0;
        match m {
            Move::Play(point) => {
                captured_stones = next_board.place_stone(self.next_player.color, &point).expect("Illegal play");

            }
            Move::Pass => {}
            Move::Resign => {}
        }

        let mut previous_states = self.previous_states.clone();
        previous_states.push((self.previous_player.color, next_board.hash()));
        let mut moves = self.moves.clone();
        moves.push(*m);

        // Clone players for next game state
        let next_player = self.previous_player.clone();
        let mut previous_player = self.next_player.clone();
        previous_player.captured += captured_stones;

        Self {
            board: next_board,
            next_player,
            previous_player,
            previous_states,
            moves
        }
    }

    fn valid_moves(&self) -> Vec<Self::Move> {
        ValidMoves::new(self)
            .into_iter()
            .collect()
    }

    fn is_valid_move(&self, the_move: &Move) -> bool {
        match the_move {
            Move::Play(point) => {
                !self.is_over() &&
                    self.board.get(&point).is_none() &&
                    !self.is_move_self_capture(self.next_player.color, the_move) &&
                    !self.does_move_violate_ko(self.next_player.color, the_move)
            }
            _ => true
        }
    }

    fn is_over(&self) -> bool {
        match self.moves.last() {
            None => false,
            Some(the_move) => match the_move {
                Move::Play(_) => false,
                // Over if two consecutive passes
                Move::Pass => match self.moves.len() {
                    1 => false,
                    _ => match self.moves.get(self.moves.len() - 2) {
                        Some(Move::Pass) => true,
                        _ => false,
                    }
                },
                Move::Resign => true
            }
        }
    }

}


pub struct ValidMoves<'a> {
    game: &'a GoState,
    points: EmptyBoardPoints<'a>,
    pass_returned: bool,
    resign_returned: bool,
}

impl<'a> ValidMoves<'a> {
    fn new(game: &'a GoState) -> Self {
        Self {
            game,
            points: game.board.empty_points(),
            pass_returned: false,
            resign_returned: false,
        }
    }
}

impl<'a> Iterator for ValidMoves<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        // Return first valid play
        while let Some(p) = self.points.next() {
            let the_move = Move::Play(p);
            if self.game.is_valid_move(&the_move) {
                return Some(the_move);
            }
        }
        // All plays handled, pass and resign left
        if !self.pass_returned && !self.game.is_over() {
            self.pass_returned = true;
            Some(Move::Pass)
        } else if !self.resign_returned {
            self.resign_returned = true;
            Some(Move::Resign)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::game::go::types::Point;
    use super::*;

    #[test]
    fn test_game_is_not_over_after_one_pass() {
        let mut game_state = GoState::new(19);
        game_state = game_state.apply_move(&Move::Play(Point::new(1, 1)));
        game_state = game_state.apply_move(&Move::Pass);
        assert!(!game_state.is_over());
    }

    #[test]
    fn test_game_is_over_after_two_consecutive_passes() {
        let mut game_state = GoState::new(19);
        game_state = game_state.apply_move(&Move::Pass);
        game_state = game_state.apply_move(&Move::Pass);
        assert!(game_state.is_over());
    }

    #[test]
    fn test_move_that_violates_ko_is_recognized() {
        let mut game_state = GoState::new(19);
        game_state = game_state.apply_move(&Move::Play(Point::new(1, 2)));  // Black 1,2
        game_state = game_state.apply_move(&Move::Play(Point::new(1, 3)));  // White 1,3
        game_state = game_state.apply_move(&Move::Play(Point::new(2, 1)));  // Black 2,1
        game_state = game_state.apply_move(&Move::Play(Point::new(2, 2)));  // White 2,2
        game_state = game_state.apply_move(&Move::Play(Point::new(3, 2)));  // Black 3,2
        game_state = game_state.apply_move(&Move::Play(Point::new(3, 3)));  // White 3,3
        game_state = game_state.apply_move(&Move::Play(Point::new(10,10))); // Black 10,10
        game_state = game_state.apply_move(&Move::Play(Point::new(2, 4)));  // White 2,4
        game_state = game_state.apply_move(&Move::Play(Point::new(2, 3)));  // Black 2,3
        assert!(game_state.does_move_violate_ko(Color::White, &Move::Play(Point::new(2, 2))));
        assert!(!game_state.does_move_violate_ko(Color::White, &Move::Play(Point::new(14, 14))));
    }

    #[test]
    fn test_self_capture() {
        let board = r#".o.
                       o.o
                       .o."#;
        let board = Board::from_str(board).unwrap();

        let game = GoState::from_board(board, Player::black());

        assert!(game.is_move_self_capture(Color::Black, &Move::Play(Point::new(2, 2))));
    }


    #[test]
    fn test_self_capture_is_not_valid_move() {
        let board = r#".o.
                       o.o
                       .o."#;
        let board = Board::from_str(board).unwrap();

        let game = GoState::from_board(board, Player::black());

        assert!(!game.is_valid_move(&Move::Play(Point::new(2, 2))));
    }

    #[test]
    fn test_move_is_not_self_capture_if_other_group_is_killed() {
        let board = r#"ooo
                       o.o
                       ooo"#;
        let board = Board::from_str(board).unwrap();

        let game = GoState::from_board(board, Player::black());

        assert!(game.is_valid_move(&Move::Play(Point::new(2, 2))));
    }


    #[test]
    fn test_is_valid_move() {
        let board = r#"
        .o.o.x.x.
        x.x.o.o.x
        .x.o.o.ox
        x.x.x.xx.
        xx.x.oo.o
        x.o.xx.x.
        .o.oo.o.o
        o.x.xx.x.
        .x.o.oxox"#;
        let board = Board::from_str(board).unwrap();

        let game = GoState::from_board(board, Player::white());

        assert!(game.is_valid_move(&Move::Play(Point::new(7, 1))));

        let board = r#"
        .xxxoo.xx
        xxxxxxxx.
        xxxxxxxxx
        .x.xx.oxx
        xxxxxxxxx
        xx.xxxxox
        xxxxoxxox
        xxxxooxoo
        xx.xoooo."#;
        let board = Board::from_str(board).unwrap();

        let game = GoState::from_board(board, Player::black());

        assert!(game.is_valid_move(&Move::Play(Point::new(1, 7))));
    }

    #[test]
    fn test_valid_moves() {
        let board = r#"
        ooo
        o.o
        oo."#;
        let board = Board::from_str(board).unwrap();

        let game = GoState::from_board(board.clone(), Player::black());
        let valid_moves = game.valid_moves();

        assert_eq!(valid_moves.len(), 2);
        assert!(valid_moves.contains(&Move::Resign));
        assert!(valid_moves.contains(&Move::Pass));

        let game = GoState::from_board(board.clone(), Player::white());
        let valid_moves = game.valid_moves();

        assert_eq!(valid_moves.len(), 4);
        assert!(valid_moves.contains(&Move::Resign));
        assert!(valid_moves.contains(&Move::Pass));
        assert!(valid_moves.contains(&Move::Play(Point::new(2, 2))));
        assert!(valid_moves.contains(&Move::Play(Point::new(3, 3))));
    }
}