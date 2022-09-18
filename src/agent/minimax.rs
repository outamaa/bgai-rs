use crate::game::GameState;

type MoveValue = i32;

#[derive(Clone, Copy, Debug)]
pub struct OptimalMove<T> {
    pub best_move: Option<T>,
    pub value: MoveValue,
}

impl<T> OptimalMove<T> {
    fn new(best_move: Option<T>, value: MoveValue) -> Self {
        Self {
            best_move,
            value
        }
    }
}

pub fn minimax<S: GameState>(game: &S, ply: u32, eval_fn: fn(&S) -> MoveValue) -> OptimalMove<S::Move> {
    // See PAIP 18.4 Searching ahead: Minimax
    if ply == 0 || game.is_over() {
        println!("FOUND WINNING MOVE!");
        return OptimalMove::new(None, eval_fn(game));
    }

    let mut results: Vec<_> =  game.valid_moves()
        .into_iter()
        .map(|the_move| {
            let next_gamestate = game.apply_move(&the_move);
            let OptimalMove {value, ..} = minimax(&next_gamestate, ply - 1, eval_fn);
            // Negate because zero-sum game => worst for opponent is best for me
            let result = OptimalMove::new(Some(the_move), -value);
            result
        })
        .collect();

    results.sort_by(|a, b| b.value.cmp(&a.value));
    println!("{:?}", results);
    results[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::one_two_three::*;

    #[test]
    fn test_one_ply_minimax_equals_selecting_move_with_best_evaluation_function_result() {
        let game = OneTwoThreeState::new();
        let optimal_move = minimax(&game, 1, score_difference);

        assert_eq!(optimal_move.best_move, Some(Move::Three));
        assert_eq!(optimal_move.value, 3);
    }

    #[test]
    fn test_two_ply_minimax_subtracts_the_opponents_best_move() {
        let game = OneTwoThreeState::new();
        let optimal_move = minimax(&game, 2, score_difference);

        assert_eq!(optimal_move.best_move, Some(Move::Three));
        assert_eq!(optimal_move.value, 0);
    }

    #[test]
    fn test_five_ply_minimax_with_one_two_three_game_finds_winning_move() {
        let game = OneTwoThreeState::new();
        let optimal_move = minimax(&game, 5, score_difference);

        assert_eq!(optimal_move.best_move, Some(Move::Three));
        assert_eq!(optimal_move.value, 3);
    }
}