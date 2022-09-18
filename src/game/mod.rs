use std::fmt::Debug;

pub mod go;
pub mod one_two_three;

pub trait GameState {
    type Move: Debug + PartialEq + Copy;

    fn apply_move(&self, m: &Self::Move) -> Self;
    fn valid_moves(&self) -> Vec<Self::Move>;
    fn is_valid_move(&self, m: &Self::Move) -> bool {
        self.valid_moves().contains(m)
    }
    fn is_over(&self) -> bool;
}