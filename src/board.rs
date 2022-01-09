use anyhow::{bail, Result};
use std::rc::Rc;
use std::str::FromStr;
use std::fmt;
use std::fmt::Formatter;

use crate::types::*;
use crate::zobrist::{ZobristHasher, ZobristHash};

#[derive(Clone, PartialEq)]
pub struct Board {
    pub rows: usize,
    pub cols: usize,
    grid: Vec<Option<Player>>,
    hasher: Rc<ZobristHasher>,
    hash: ZobristHash,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Self {
            rows: size,
            cols: size,
            grid: vec![None; size * size],
            hasher: Rc::new(ZobristHasher::new(size)),
            hash: ZobristHasher::empty_board()
        }
    }

    pub fn place_stone(&mut self, player: Player, point: &Point) -> Result<usize> {
        if self.get(point).is_some() {
            bail!("{:?} is not empty", point);
        }
        self.apply_hash_for_play(player, point);
        self.set(point, Some(player));
        let mut captured_points= Vec::new();
        // Assume the move is not self-capture, remove opponent's adjacent groups
        // that ran out of liberties
        for neighbor in point
            .neighbors()
            .iter()
            .filter(|p| self.is_on_grid(p) && self.get(p) == Some(player.other())) {
            captured_points = self.group_without_liberties(neighbor, captured_points);
        }
        for captured_point in &captured_points {
            self.remove_stone(captured_point);
        }

        Ok(captured_points.len())
    }

    fn remove_stone(&mut self, captured_point: &Point) {
        // Assume this is only called for point with stone
        let player = self.get(captured_point).expect(&format!("Failed to remove stone at point {:?}", captured_point));
        self.apply_hash_for_play(player, captured_point);
        self.set(captured_point, None);
    }

    /// Return a group with no liberties containing the point, or empty vec if
    /// the group has even one liberty
    fn group_without_liberties(&self, point: &Point, mut captured: Vec<Point>) -> Vec<Point> {
        let color = self.get(point).unwrap(); // Should be called only on points with a stone
        let mut unexplored = vec![*point];
        let mut explored = Vec::new();

        while let Some(point) = unexplored.pop() {
            if !captured.contains(&point) {
                explored.push(point);
            }
            for neighbor in point.neighbors().iter().filter(|p| self.is_on_grid(p)) {
                match self.get(&neighbor) {
                    None => {
                        // The group has at least one liberty, return previously captured
                        return captured;
                    },
                    Some(neighbor_color) => {
                        // Ignore opponent's stones and stones that are already added to group
                        if neighbor_color == color
                            && !captured.contains(&neighbor)
                            && !explored.contains(&neighbor)
                            && !unexplored.contains(&neighbor) {
                            unexplored.push(*neighbor);
                        }
                    }
                }
            }
        }

        captured.append(&mut explored);
        captured
    }

    pub fn is_alive(&self, point: &Point) -> bool {
        assert!(self.get(point).is_some());
        self.group_without_liberties(point, Vec::new()).len() == 0
    }

    pub fn is_eye(&self, point: &Point, color: Player) -> bool {
        match self.get(point) {
            None => {
                for neighbor in point.neighbors() {
                    if self.is_on_grid(&neighbor) {
                        if self.get(&neighbor) != Some(color) {
                            return false;
                        }
                    }
                }
                let mut friendly_corners = 0;
                let mut off_board_corners = 0;
                for corner in point.diagonals() {
                    if self.is_on_grid(&corner) {
                        let corner_color = self.get(&corner);
                        if corner_color == Some(color) {
                            friendly_corners += 1;
                        }
                    } else {
                        off_board_corners += 1;
                    }
                }
                if off_board_corners > 0 {
                    off_board_corners + friendly_corners == 4
                } else {
                    friendly_corners >= 3
                }
            }
            // Point can't be eye if there's a stone
            Some(_) => false
        }
    }

    fn is_on_grid(&self, point: &Point) -> bool {
        (1..=self.rows).contains(&point.row) && (1..=self.cols).contains(&point.col)
    }

    pub fn get(&self, point: &Point) -> Option<Player> {
        assert!(self.is_on_grid(point));
        self.grid[(point.row - 1) * self.cols + (point.col - 1)]
    }

    fn set(&mut self, point: &Point, value: Option<Player>) {
        self.grid[(point.row - 1) * self.cols + (point.col - 1)] = value;
    }

    pub fn hash(&self) -> ZobristHash {
        self.hash
    }

    fn apply_hash_for_play(&mut self, player: Player, point: &Point) {
        self.hash = self.hasher.hash_move(self.hash, player, point);
    }
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let lines: Vec<&str> = s
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.trim())
            .collect();
        let rows = lines.len();
        let cols = lines[0].len();
        assert_eq!(rows, cols);

        let mut board = Self::new(rows);

        for (row_idx, row) in lines.iter().enumerate() {
            for (col_idx, c) in row.chars().filter(|c| !c.is_whitespace()).enumerate() {
                let point = Point::new(row_idx + 1, col_idx + 1);
                let contents = match c {
                    'o' => Some(Player::White),
                    'x' => Some(Player::Black),
                    '.' => None,
                    c => bail!("Invalid character: {}", c)
                };

                if let Some(player) = contents {
                    board.apply_hash_for_play(player, &point);
                }
                board.set(&point, contents);
            }
        }

        Ok(board)
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "  ")?;
        for i in 1..=self.cols {
            write!(f, " {:2}", i)?;
        }
        write!(f, "\n")?;

        for row in 1..=self.rows {
            write!(f, "{:2} ", row)?;
            for col in 1..=self.cols {
                let contents = self.get(&Point::new(row, col));
                let c = match contents {
                    None => '.',
                    Some(color) => match color {
                        Player::Black => 'x',
                        Player::White => 'o'
                    }
                };
                write!(f, " {} ", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in 1..=self.rows {
            for col in 1..=self.cols {
                let contents = self.get(&Point::new(row, col));
                let c = match contents {
                    None => '.',
                    Some(color) => match color {
                        Player::Black => 'x',
                        Player::White => 'o'
                    }
                };
                write!(f, "{}", c);
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Board, Point, Player};
    use std::str::FromStr;

    #[test]
    fn test_stone_with_liberties_is_alive() {
        let board = r#".o.
                             .x.
                             .o."#;
        let board = Board::from_str(board).unwrap();
        assert!(board.is_alive(&Point::new(2, 2)));
    }

    #[test]
    fn test_stone_without_liberties_is_dead() {
        let board = r#".o.
                             oxo
                             .o."#;
        let board = Board::from_str(board).unwrap();
        assert!(!board.is_alive(&Point::new(2, 2)));
    }

    #[test]
    fn test_stone_in_corner_with_liberties_is_alive() {
        let board = r#"xo
                             .."#;
        let board = Board::from_str(board).unwrap();
        assert!(board.is_alive(&Point::new(1, 1)));
    }

    #[test]
    fn test_stone_in_corner_without_liberties_is_dead() {
        let board = r#".o
                             ox"#;
        let board = Board::from_str(board).unwrap();
        assert!(!board.is_alive(&Point::new(2, 2)));
    }

    #[test]
    fn test_placing_and_removing_stone_preserves_hash() {
        let mut board = Board::new(19);
        let original_hash = board.hash();
        board.place_stone(Player::White, &Point::new(1, 1));
        assert_ne!(original_hash, board.hash(), "Placing stones changes the hash");
        board.remove_stone(&Point::new(1, 1));
        assert_eq!(original_hash, board.hash(), "Removing stone reverts the hash");
    }

    #[test]
    fn test_placing_and_removing_many_stones_preserves_hash() {
        let mut board = Board::new(19);
        let original_hash = board.hash();

        // Add multiple stones
        let _ = board.place_stone(Player::White, &Point::new(1, 1));
        let _ = board.place_stone(Player::White, &Point::new(2, 1));
        let _ = board.place_stone(Player::White, &Point::new(3, 1));
        let _ = board.place_stone(Player::Black, &Point::new(11, 1));
        let _ = board.place_stone(Player::Black, &Point::new(12, 1));
        let _ = board.place_stone(Player::Black, &Point::new(13, 1));

        // Remove in shuffled order
        board.remove_stone(&Point::new(2, 1));
        board.remove_stone(&Point::new(12, 1));
        board.remove_stone(&Point::new(3, 1));
        board.remove_stone(&Point::new(11, 1));
        board.remove_stone(&Point::new(1, 1));
        board.remove_stone(&Point::new(13, 1));
        
        assert_eq!(original_hash, board.hash(), "Removing stone reverts the hash");
    }

    #[test]
    fn test_is_eye_without_off_board_corners() {
        let board = r#".oo
                             o.o
                             .o."#;
        let board = Board::from_str(board).unwrap();
        assert!(!board.is_eye(&Point::new(1, 2), Player::White));
        assert!(!board.is_eye(&Point::new(2, 2), Player::White));
        assert!(!board.is_eye(&Point::new(2, 2), Player::Black));

        let board = r#"ooo
                             o.o
                             .o."#;
        let board = Board::from_str(board).unwrap();
        assert!(!board.is_eye(&Point::new(2, 2), Player::White));
        assert!(!board.is_eye(&Point::new(2, 2), Player::Black));

        let board = r#"ooo
                             o.o
                             .oo"#;
        let board = Board::from_str(board).unwrap();
        assert!(board.is_eye(&Point::new(2, 2), Player::White));
        assert!(!board.is_eye(&Point::new(2, 2), Player::Black));

        let board = r#"ooo
                             o.o
                             ooo"#;
        let board = Board::from_str(board).unwrap();
        assert!(board.is_eye(&Point::new(2, 2), Player::White));
        assert!(!board.is_eye(&Point::new(2, 2), Player::Black));
    }

    #[test]
    fn test_is_eye_with_off_board_corners() {
        let board = r#".x.
                             ..x
                             ..."#;
        let board = Board::from_str(board).unwrap();
        assert!(!board.is_eye(&Point::new(1, 3), Player::Black));
        assert!(!board.is_eye(&Point::new(1, 3), Player::White));

        let board = r#".x.
                             .xx
                             ..."#;
        let board = Board::from_str(board).unwrap();
        assert!(board.is_eye(&Point::new(1, 3), Player::Black));
        assert!(!board.is_eye(&Point::new(1, 3), Player::White));

        let board = r#"xx.
                             .x.
                             x.."#;
        let board = Board::from_str(board).unwrap();
        assert!(!board.is_eye(&Point::new(2, 1), Player::Black));
        assert!(!board.is_eye(&Point::new(2, 1), Player::White));

        let board = r#"xx.
                             .x.
                             xx."#;
        let board = Board::from_str(board).unwrap();
        assert!(board.is_eye(&Point::new(2, 1), Player::Black));
        assert!(!board.is_eye(&Point::new(2, 1), Player::White));

        let board = r#"xx.
                             .x.
                             xo."#;
        let board = Board::from_str(board).unwrap();
        assert!(!board.is_eye(&Point::new(2, 1), Player::Black));
        assert!(!board.is_eye(&Point::new(2, 1), Player::White));

    }
}

