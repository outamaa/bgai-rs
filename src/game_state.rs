use crate::{Board, Player, Move};

#[derive(Debug, Clone, PartialEq)]
struct GameState {
    board: Board,
    next_player: Player,
    previous_state: Box<Option<GameState>>,
    last_move: Option<Move>,
}

impl GameState {
    fn new(board_size: usize) -> Self {
        Self {
            board: Board::new(board_size),
            next_player: Player::Black,
            previous_state: Box::new(None),
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

        Self {
            board: next_board,
            next_player: self.next_player.other(),
            previous_state: Box::new(Some(self.clone())),
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
                    .previous_state
                    .as_ref().as_ref()
                    .map(|state| state
                        .last_move
                        .map(|previous_move| previous_move == Move::Pass)
                        .unwrap_or(false))
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