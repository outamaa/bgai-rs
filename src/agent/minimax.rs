use crate::{GameState, Move};

type MoveValue = i32;

fn minimax(game: &GameState, ply: u32, eval_fn: fn(&GameState) -> (Move, MoveValue)) -> (Move, MoveValue) {
    // See PAIP 18.4 Searching ahead: Minimax
    if ply == 0 {
        return eval_fn(game);
    }

    game.valid_moves()
        .map(|the_move| {
            let next_gamestate = game.apply_move(the_move);
            let (best_move, value) = minimax(&next_gamestate, ply - 1, eval_fn);
            // Negate because zero-sum game => worst for opponent is best for me
            (best_move, -value)
        })
        // Find the best (Move, MoveValue) tuple by maximizing MoveValue
        .max_by(|a, b| a.1.cmp(&b.1))
        .expect("No valid moves")
}