/// Common types needed everywhere

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn other(&self) -> Player {
        if *self == Self::Black {
            Self::White
        } else {
            Self::Black
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub(crate) fn neighbors(&self) -> Vec<Point> {
        vec![
            Self::new(self.row - 1, self.col),
            Self::new(self.row + 1, self.col),
            Self::new(self.row, self.col - 1),
            Self::new(self.row, self.col + 1),
        ]
    }

    pub(crate) fn diagonals(&self) -> Vec<Point> {
        vec![
            Self::new(self.row - 1, self.col - 1),
            Self::new(self.row + 1, self.col - 1),
            Self::new(self.row - 1, self.col + 1),
            Self::new(self.row + 1, self.col + 1),
        ]

    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Move {
    Play(Point),
    Pass,
    Resign,
}
