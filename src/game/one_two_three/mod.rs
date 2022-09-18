//! One-Two-Three, a simple (and exciting!) game where
//! - players in turn choose numbers one, two or three
//! - the score of a player is the sum of numbers they have chosen
//! - the first player to reach 9 points wins
//!
//! The motivation for the game is to provide a sanity check for the different
//! AI strategies, since 1) choosing three points is always the correct play
//! and 2) the first player always wins if they play correctly.

use crate::GameState;

#[derive(Clone, Debug)]
pub struct OneTwoThreeState {
    players: [Player; 2],
    current_player_index: usize,
}

impl OneTwoThreeState {
    pub fn new() -> Self {
        Self { players: [Player::new(), Player::new()], current_player_index: 0 }
    }

    fn next_player_index(&self) -> usize {
        if self.current_player_index == 0 {
            1
        } else {
            0
        }
    }

    fn switch_current_player(&mut self) {
        self.current_player_index = self.next_player_index();
    }

    pub fn current_player(&self) -> &Player {
        self.players.get(self.current_player_index).unwrap()
    }

    pub fn next_player(&self) -> &Player {
        self.players.get(self.next_player_index()).unwrap()
    }

    fn current_player_mut(&mut self) -> &mut Player {
        self.players.get_mut(self.current_player_index).unwrap()
    }
}

impl GameState for OneTwoThreeState {
    type Move = Move;

    fn apply_move(&self, m: &Self::Move) -> Self {
        let mut new_state = self.clone();
        new_state.current_player_mut().points += match m {
            Move::One => 1,
            Move::Two => 2,
            Move::Three => 3,
        };
        new_state.switch_current_player();

        new_state
    }

    fn valid_moves(&self) -> Vec<Self::Move> {
        vec![
            Move::One,
            Move::Two,
            Move::Three,
        ]
    }

    fn is_over(&self) -> bool {
        self.players.iter().any(|p| p.points >= 9)
    }
}

#[derive(Clone, Debug)]
pub struct Player {
    points: u32
}

impl Player {
    fn new() -> Self {
        Self { points: 0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Move {
    One,
    Two,
    Three
}

/// Evaluation function calculating the difference between the scores of the
/// current player and the other player
pub fn score_difference(game: &OneTwoThreeState) -> i32 {
    game.current_player().points as i32 - game.next_player().points as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_difference() {
        let mut game = OneTwoThreeState {
            players: [
                Player {
                    points: 4
                },
                Player {
                    points: 5
                }
            ],
            current_player_index: 0
        };

        assert_eq!(score_difference(&game), -1);

        game.switch_current_player();

        assert_eq!(score_difference(&game), 1);
    }
}