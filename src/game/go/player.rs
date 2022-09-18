use crate::game::go::types::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub color: Color,
    pub captured: usize,
}

impl Player {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            captured: 0,
        }
    }
    pub fn white() -> Self {
        Self::new(Color::White)
    }

    pub fn black() -> Self {
        Self::new(Color::Black)
    }

}