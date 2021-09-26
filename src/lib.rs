use anyhow::{bail, Result};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::rc::Rc;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Player {
    Black,
    White,
}

impl Player {
    fn other(&self) -> Player {
        if *self == Self::Black {
            Self::White
        } else {
            Self::Black
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn neighbors(&self) -> Vec<Point> {
        vec![
            Self::new(self.row - 1, self.col),
            Self::new(self.row + 1, self.col),
            Self::new(self.row, self.col - 1),
            Self::new(self.row, self.col + 1),
        ]
    }
}

#[derive(Copy, Clone, Debug)]
enum Move {
    Play(Point),
    Pass,
    Resign,
}

#[derive(Clone, Debug, PartialEq)]
struct GoString {
    color: Player,
    stones: HashSet<Point>,
    liberties: HashSet<Point>,
}

impl GoString {
    fn remove_liberty(&mut self, point: &Point) {
        self.liberties.remove(point);
    }

    fn add_liberty(&mut self, point: &Point) {
        self.liberties.insert(*point);
    }

    fn merged_with(&self, other: &GoString) -> Self {
        assert_eq!(self.color, other.color);
        let combined_stones: HashSet<&Point> = self.stones.union(&other.stones).collect();
        let liberties = self
            .liberties
            .union(&other.liberties)
            .filter(|&p| !combined_stones.contains(p));

        Self {
            color: self.color,
            stones: combined_stones.iter().map(|&x| *x).collect(),
            liberties: liberties.into_iter().cloned().collect(),
        }
    }

    fn num_liberties(&self) -> usize {
        self.liberties.len()
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Board {
    rows: usize,
    cols: usize,
    grid: Vec<Option<Player>>,
}

impl Board {
    fn new(size: usize) -> Self {
        Self {
            rows: size,
            cols: size,
            grid: vec![None; size * size],
        }
    }

    fn place_stone(&mut self, player: Player, point: &Point) -> Result<()> {
        if self.get(point).is_some() {
            bail!("{:?} is not empty", point);
        }
        self.grid[(point.row - 1) * self.cols + (point.col - 1)] = Some(player);
        for neighbor in point.neighbors().iter().filter(|p| self.get(p).is_some()) {
            // build strings
            // combine remove strings with no liberties
        }

        Ok(())
    }

    fn is_on_grid(&self, point: &Point) -> bool {
        (1..=self.rows).contains(&point.row) && (1..=self.cols).contains(&point.col)
    }

    fn get(&self, point: &Point) -> Option<Player> {
        assert!(self.is_on_grid(point));
        self.grid[(point.row - 1) * self.cols + (point.col - 1)]
    }
}
