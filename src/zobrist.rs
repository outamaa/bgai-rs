use rand::prelude::*;
use crate::{Player, Point};
use std::fmt;
use std::fmt::Formatter;

pub struct ZobristHasher {
    seed: u64,
    board_size: usize,
    lut: Vec<u64>
}

pub type ZobristHash = u64;

impl ZobristHasher {
    pub fn new(board_size: usize) -> Self {
        const DEFAULT_SEED: u64 = 1985;
        const MAX63: u64 = 0x7fffffffffffffff;
        let mut lut = vec![0u64; board_size * board_size * 2];
        let mut rng = rand_pcg::Pcg64::seed_from_u64(DEFAULT_SEED);
        for hash in &mut lut {
            *hash = rng.gen_range(0..MAX63);
        }

        Self {
            seed: DEFAULT_SEED,
            board_size,
            lut
        }
    }

    pub fn empty_board() -> ZobristHash {
        0
    }

    pub fn hash_move(&self, hash: ZobristHash, player: Player, point: &Point) -> ZobristHash {
        let offset = match player {
            Player::Black => 0,
            Player::White => 1,
        };
        let index = (point.row - 1) * self.board_size + (point.col - 1) + offset;

        hash ^ self.lut[index]
    }
}

impl fmt::Debug for ZobristHasher {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ZobristHasher {{ board_size: {}, lut[...] }}", self.board_size)
    }
}

impl PartialEq for ZobristHasher {
    fn eq(&self, other: &Self) -> bool {
        self.board_size == other.board_size && self.seed == other.seed
    }
}