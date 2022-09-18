pub mod board;
pub mod types;
pub mod state;
pub mod zobrist;
pub mod player;

pub use board::Board;
pub use types::{Point, Color, Move};
pub use state::GoState;
pub use player::Player;


/// A test evaluation function
pub fn stone_difference(game: &GoState) -> i32 {
    let last_move = game.moves.last();
    let position_score = match last_move {
        None => 0,
        Some(the_move) => match the_move {
            Move::Play(p) => if game.board.is_on_edge(p) {
                10 // a simple heuristic, previous player usually shouldn't have played on edge, thus it's beneficial for next player
            } else {
                0
            }
            Move::Pass => 50,
            Move::Resign => i32::MAX/2
        }
    };
    let final_score = final_score(game);
    final_score + position_score
}

/// A todo final score, (captured + own stones on board) of next player - (captured + own stones on board) for previous player
fn final_score(game: &GoState) -> i32 {
    let previous_player_eval = (game.previous_player.captured + game.board.number_of_stones_of_color(game.previous_player.color)) as i32;
    let next_player_eval = (game.next_player.captured + game.board.number_of_stones_of_color(game.next_player.color)) as i32;
    let result = next_player_eval - previous_player_eval;
    result
}
