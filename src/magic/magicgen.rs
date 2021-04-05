use crate::components::bitboard::{
    AddPiece, Bitboard, ClearBit, GetBit, FILEA, FILEB, FILEC, FILED, FILEE, FILEF, FILEG, FILEH,
    RANK1, RANK2, RANK3, RANK4, RANK5, RANK6, RANK7, RANK8,
};
use crate::components::piece;
use crate::components::piece::PieceType;
use crate::components::square::SquareIndex::A1;
use crate::components::square::{rank_file_to_index, Square};
use crate::magic::util::{
    bishop_attacks, bishop_ray, occupancy, rook_attacks, rook_ray, MagicPiece,
};
use itertools::{all, Combinations, Itertools};
use rand::rngs::ThreadRng;
use rand::{Rng, RngCore};
use rayon::prelude::IntoParallelIterator;
use std::slice::Iter;

static ROOK_RELEVANT_BITS: [usize; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 12, 11, 11, 11, 11, 11, 11, 12,
];

static BISHOP_RELEVANT_BITS: [usize; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 6,
];

pub struct MagicTable {
    pub table: Vec<u64>,
    pub magics: Vec<u64>,
}

impl MagicTable {
    pub fn init(piece: MagicPiece) -> MagicTable {
        let len = match piece {
            MagicPiece::Rook => ROOK_RELEVANT_BITS.iter().map(|x| 1 << *x).sum(),
            MagicPiece::Bishop => BISHOP_RELEVANT_BITS.iter().map(|x| 1 << *x).sum(),
        };
        let mut table = vec![0; len];

        let mut magics: Vec<u64> = Vec::with_capacity(64);
        let mut random = ThreadRng::default();

        let mut start_index = 0;
        for i in 0..64 {
            let end_index = match piece {
                MagicPiece::Rook => start_index + (1 << ROOK_RELEVANT_BITS[i]),
                MagicPiece::Bishop => start_index + (1 << BISHOP_RELEVANT_BITS[i]),
            };
            let mut m = MagicGenerator::new(
                i as u8,
                piece,
                &mut random,
                &mut table[start_index..end_index],
            );
            start_index = end_index;
            let magic = m.find_magic_number();
            magics.push(magic);
        }

        MagicTable { table, magics }
    }

    pub fn moves(&self, square: Square, blockers: Bitboard) -> Bitboard {
        let mask = rook_ray(square);
        let occupancy = mask & blockers;
        let bits = ROOK_RELEVANT_BITS[square as usize];
        let magic = self.magics[square as usize];

        let key = MagicGenerator::key(occupancy, magic, bits);

        self.table[key]
    }
}

pub struct MagicGenerator<'a> {
    occupancies: [Bitboard; 4096],
    table: &'a mut [u64],
    attack_map: [Bitboard; 4096],
    bits: usize,
    random: &'a mut ThreadRng,
}

impl MagicGenerator<'_> {
    pub fn new<'a>(
        square: Square,
        piece: MagicPiece,
        random: &'a mut ThreadRng,
        table: &'a mut [u64],
    ) -> MagicGenerator<'a> {
        let bits: usize = match piece {
            MagicPiece::Rook => ROOK_RELEVANT_BITS,
            MagicPiece::Bishop => BISHOP_RELEVANT_BITS,
        }[square as usize];
        let mut occupancies: [Bitboard; 4096] = [0; 4096];
        let mut attack_map: [Bitboard; 4096] = [0; 4096];

        let ray = match piece {
            MagicPiece::Rook => rook_ray(square),
            MagicPiece::Bishop => bishop_ray(square),
        };

        for i in 0..(1 << bits) {
            occupancies[i] = occupancy(i, bits, ray);
        }

        for i in 0..(1 << bits) {
            match piece {
                MagicPiece::Rook => attack_map[i] = rook_attacks(square, occupancies[i]),
                MagicPiece::Bishop => attack_map[i] = bishop_attacks(square, occupancies[i]),
            }
        }

        MagicGenerator {
            occupancies,
            table,
            attack_map,
            bits,
            random,
        }
    }

    pub fn key(occupied: Bitboard, magic: u64, bits: usize) -> usize {
        (occupied.wrapping_mul(magic) >> (64 - bits)) as usize
    }

    pub fn find_magic_number(&mut self) -> u64 {
        for k in 0..1000000 {
            let magic = self.gen_random_number();
            let mut fail = false;
            self.table.iter_mut().for_each(|m| *m = 0);
            let mut i = 0;
            'inner: while i < (1 << self.bits) {
                let occupied = self.occupancies[i];
                let key = MagicGenerator::key(occupied, magic, self.bits);
                if self.table[key] == 0 {
                    self.table[key] = self.attack_map[i];
                } else if self.table[key] != self.attack_map[i] {
                    fail = true;
                    break 'inner;
                }
                i += 1;
            }
            if !fail {
                println!("it took {} iterations", k);
                return magic;
            }
        }
        0
    }

    fn gen_random_number(&mut self) -> u64 {
        let n1: u64 = self.gen_u64();
        let n2: u64 = self.gen_u64();
        let n3: u64 = self.gen_u64();
        n1 & n2 & n3
    }

    fn gen_u64(&mut self) -> u64 {
        let u1: u64 = self.random.next_u64() & 0xFFFF;
        let u2: u64 = self.random.next_u64() & 0xFFFF;
        let u3: u64 = self.random.next_u64() & 0xFFFF;
        let u4: u64 = self.random.next_u64() & 0xFFFF;
        u1 | (u2 << 16) | (u3 << 32) | (u4 << 48)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::square::SquareIndex::{A1, A8, B2, C7, D4, H1, H8};

    #[test]
    fn correct_combinations() {
        let mut rng = ThreadRng::default();
        //let mut b = MagicGenerator::new(10, MagicPiece::Rook, &mut rng);
        //let m = b.find_magic_number();
        //assert_eq!(m.0, 756607761056301088);
    }
}
