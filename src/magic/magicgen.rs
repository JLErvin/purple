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
        //let len = match piece {
        //    MagicPiece::Rook => ROOK_RELEVANT_BITS.iter().map(|x| 1 << *x).sum(),
        //    MagicPiece::Bishop => BISHOP_RELEVANT_BITS.iter().map(|x| 1 << *x).sum(),
        //};
        let len = 0;
        let mut table: Vec<u64> = Vec::with_capacity(len);
        let magics: Vec<u64> = Vec::with_capacity(64);
        let mut random = ThreadRng::default();

        for i in 0..64 {
            let mut m = MagicGenerator::new(i, MagicPiece::Rook, &mut random);
            let mut magic = m.find_magic_number();
            //table.push(magic.0);
            //table.append(&mut magic.1);
        }

        MagicTable { table, magics }
    }
}

pub struct MagicGenerator<'a> {
    occupancies: [Bitboard; 4096],
    attack_map: [Bitboard; 4096],
    used_map: [Bitboard; 4096],
    bits: usize,
    random: &'a mut ThreadRng,
}

impl MagicGenerator<'_> {
    pub fn new(square: Square, piece: MagicPiece, random: &mut ThreadRng) -> MagicGenerator {
        let bits: usize = match piece {
            MagicPiece::Rook => unsafe { ROOK_RELEVANT_BITS },
            MagicPiece::Bishop => unsafe { BISHOP_RELEVANT_BITS },
        }[square as usize];
        let mut occupancies: [Bitboard; 4096] = [0; 4096];
        let mut attack_map: [u64; 4096] = [0; 4096];
        let used_map: [u64; 4096] = [0; 4096];

        let ray = match piece {
            MagicPiece::Rook => rook_ray(square),
            MagicPiece::Bishop => bishop_ray(square),
        };

        for i in 0..4096 {
            occupancies[i] = occupancy(i, bits, ray);
        }

        for i in 0..4096 {
            match piece {
                MagicPiece::Rook => attack_map[i] = rook_attacks(square, occupancies[i]),
                MagicPiece::Bishop => attack_map[i] = bishop_attacks(square, occupancies[i]),
            }
        }

        MagicGenerator {
            occupancies,
            attack_map,
            used_map,
            bits,
            random,
        }
    }

    fn key(occupied: Bitboard, magic: u64, bits: usize) -> usize {
        (occupied.wrapping_mul(magic) >> (64 - bits)) as usize
    }

    pub fn find_magic_number(&mut self) -> (Bitboard, Vec<u64>) {
        for k in 0..1000000 {
            let magic = self.gen_random_number();
            let mut fail = false;
            self.used_map.iter_mut().for_each(|m| *m = 0);
            let mut i = 0;
            'inner: while i < 4096 {
                let occupied = self.occupancies[i];
                let key = MagicGenerator::key(occupied, magic, self.bits);
                if self.used_map[key] == 0 {
                    self.used_map[key] = self.attack_map[i];
                } else if self.used_map[key] != self.attack_map[i] {
                    fail = true;
                    break 'inner;
                }
                i += 1;
            }
            if !fail {
                println!("it took {} iterations", k);
                let v: Vec<u64> = Vec::new();
                //let mut v: Vec<u64> = Vec::with_capacity(1 << self.bits);
                //for i in 0..(1 << self.bits) {
                //    v.push(self.used_map[i]);
                //}
                return (magic, v);
            }
        }
        (0, Vec::new())
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
        let mut b = MagicGenerator::new(10, MagicPiece::Rook, &mut rng);
        let m = b.find_magic_number();
        assert_eq!(m.0, 756607761056301088);
    }
}
