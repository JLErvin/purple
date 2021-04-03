use crate::components::bitboard::{
    AddPiece, Bitboard, ClearBit, GetBit, FILEA, FILEB, FILEC, FILED, FILEE, FILEF, FILEG, FILEH,
    RANK1, RANK2, RANK3, RANK4, RANK5, RANK6, RANK7, RANK8,
};
use crate::components::piece;
use crate::components::piece::PieceType;
use crate::components::square::SquareIndex::A1;
use crate::components::square::{rank_file_to_index, Square};
use itertools::{all, Combinations, Itertools};
use rand::rngs::ThreadRng;
use rand::{Rng, RngCore};
use std::slice::Iter;

static mut ROOK_RELEVANT_BITS: [usize; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 12, 11, 11, 11, 11, 11, 11, 12,
];

static mut BISHOP_RELEVANT_BITS: [usize; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 6,
];

pub struct MagicManager {
    rooks: [Bitboard; 64],
    bishops: [Bitboard; 64],
}

pub struct MagicGenerator<'a> {
    occupancies: [Bitboard; 4096],
    attack_map: [Bitboard; 4096],
    used_map: [Bitboard; 4096],
    bits: usize,
    random: &'a mut ThreadRng,
}

impl MagicGenerator<'_> {
    /// Generates a new MagicGenerator which is used to search for the magic number
    /// associated with the given square and PieceType (i.e. Rook or Bishop)
    pub fn new(square: Square, piece: PieceType, random: &mut ThreadRng) -> MagicGenerator {
        let mut occupancies: [Bitboard; 4096] = [0; 4096];
        let mut attack_map: [u64; 4096] = [0; 4096];
        let used_map: [u64; 4096] = [0; 4096];
        let bits = unsafe { ROOK_RELEVANT_BITS }[square as usize];
        let rook_ray = MagicGenerator::gen_rook_attacks_empty_board(square);

        for i in 0..4096 {
            occupancies[i] = MagicGenerator::gen_occupancy(i, bits, rook_ray);
        }

        for i in 0..4096 {
            attack_map[i] = MagicGenerator::gen_legal_attacks(A1 as u8, occupancies[i]);
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

    pub fn find_magic_number(&mut self) -> u64 {
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
                //println!("it took {} iterations", k);
                return magic;
            }
        }
        0
    }

    /// Given a square and a bitboard representing all blockers for that position returns
    /// a new Bitboard which represents all possible moves the slider can make in that position.
    fn gen_legal_attacks(square: Square, blockers: Bitboard) -> Bitboard {
        let mut b: Bitboard = 0;
        let rank = square / 8;
        let file = square % 8;
        let rank_bb = MagicGenerator::rank_to_bb(rank);
        let file_bb = MagicGenerator::file_to_bb(file);
        let relevant_blockers: Bitboard = (file_bb | rank_bb) & blockers;

        // in the NORTH direction
        for i in (rank + 1)..8 {
            b = b.add_piece(i, file);
            let s = rank_file_to_index(i, file);
            if relevant_blockers.get_bit_lsb(s as i8) {
                break;
            }
        }

        // in the EAST direction
        for i in (file + 1)..8 {
            b = b.add_piece(rank, i);
            let s = rank_file_to_index(rank, i);
            if relevant_blockers.get_bit_lsb(s as i8) {
                break;
            }
        }

        // in the SOUTH direction
        for i in (0..rank).rev() {
            b = b.add_piece(i, file);
            let s = rank_file_to_index(i, file);
            if relevant_blockers.get_bit_lsb(s as i8) {
                break;
            }
        }

        // in the WEST direction
        for i in (0..file).rev() {
            b = b.add_piece(rank, i);
            let s = rank_file_to_index(rank, i);
            if relevant_blockers.get_bit_lsb(s as i8) {
                break;
            }
        }

        b
    }

    fn gen_occupancy(
        occupancy_index: usize,
        relevant_bits: usize,
        mut attack_mask: Bitboard,
    ) -> Bitboard {
        let mut b: Bitboard = 0;

        for index in 0..relevant_bits {
            let square = attack_mask.trailing_zeros() as Bitboard;
            attack_mask = attack_mask.clear_bit(square as u8);
            if occupancy_index & (1 << index) != 0 {
                b |= 1 << square
            }
        }

        b
    }

    /// Given an input square, generate all possible rook attacks on the empty board,
    /// excluding both the specified input square and the border squares.
    /// For example, an input of 1 (i.e. the square A1) would generate a bitboard
    /// with occupancies on b1->b7 and a2->7 with a decimal value of 282578800148862.
    fn gen_rook_attacks_empty_board(square: Square) -> Bitboard {
        let mut square_bb: Bitboard = 0;
        square_bb = square_bb.add_at_square(square);
        let mut b: Bitboard = 0;
        let rank = square / 8;
        let file = square % 8;

        let rank_bb = MagicGenerator::rank_to_bb(rank);
        let file_bb = MagicGenerator::file_to_bb(file);

        b = rank_bb | file_bb;
        b = b & !square_bb;
        if file != 0 {
            b &= !FILEA;
        }
        if file != 7 {
            b &= !FILEH;
        }
        if rank != 0 {
            b &= !RANK1;
        }
        if rank != 7 {
            b &= !RANK8;
        }
        b
    }

    fn rank_to_bb(rank: u8) -> Bitboard {
        match rank {
            0 => RANK1,
            1 => RANK2,
            2 => RANK3,
            3 => RANK4,
            4 => RANK5,
            5 => RANK6,
            6 => RANK7,
            7 => RANK8,
            _ => 0,
        }
    }

    fn file_to_bb(file: u8) -> Bitboard {
        match file {
            0 => FILEA,
            1 => FILEB,
            2 => FILEC,
            3 => FILED,
            4 => FILEE,
            5 => FILEF,
            6 => FILEG,
            7 => FILEH,
            _ => 0,
        }
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
    use crate::components::square::SquareIndex::{A1, A8, B2, D4, H1, H8};

    #[test]
    fn correct_attack_board_a1() {
        let b = MagicGenerator::gen_rook_attacks_empty_board(A1 as u8);
        assert_eq!(b, 282578800148862);
    }

    #[test]
    fn correct_attack_board_b2() {
        let b = MagicGenerator::gen_rook_attacks_empty_board(B2 as u8);
        assert_eq!(b, 565157600328704);
    }

    #[test]
    fn correct_attack_board_d4() {
        let b = MagicGenerator::gen_rook_attacks_empty_board(D4 as u8);
        assert_eq!(b, 2260632246683648);
    }

    #[test]
    fn correct_attack_board_h1() {
        let b = MagicGenerator::gen_rook_attacks_empty_board(H1 as u8);
        assert_eq!(b, 36170086419038334);
    }

    #[test]
    fn correct_attack_board_h8() {
        let b = MagicGenerator::gen_rook_attacks_empty_board(H8 as u8);
        assert_eq!(b, 9115426935197958144);
    }

    #[test]
    fn correct_attack_board_a8() {
        let b = MagicGenerator::gen_rook_attacks_empty_board(A8 as u8);
        assert_eq!(b, 9079539427579068672);
    }

    #[test]
    fn correct_legal_attacks_d4() {
        let blockers: Bitboard = 576460752303423488;
        let b = MagicGenerator::gen_legal_attacks(D4 as u8, blockers);
        assert_eq!(b, 578721386714368008);

        let blockers: Bitboard = 8797200320512;
        let b = MagicGenerator::gen_legal_attacks(D4 as u8, blockers);
        assert_eq!(b, 8832432998400);
    }

    #[test]
    fn correct_combinations() {
        let mut rng = ThreadRng::default();
        let mut b = MagicGenerator::new(10, PieceType::Rook, &mut rng);
        let m = b.find_magic_number();
        assert_eq!(m, 756607761056301088);
    }
}
