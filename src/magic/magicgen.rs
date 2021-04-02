use crate::components::bitboard::{
    AddPiece, Bitboard, ClearBit, GetBit, FILEA, FILEB, FILEC, FILED, FILEE, FILEF, FILEG, FILEH,
    RANK1, RANK2, RANK3, RANK4, RANK5, RANK6, RANK7, RANK8,
};
use crate::components::square::SquareIndex::A1;
use crate::components::square::{rank_file_to_index, Square};
use itertools::{all, Combinations, Itertools};
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

pub struct MagicGenerator {
    occupancies: [Bitboard; 4096],
    attack_map: [Bitboard; 4096],
    used_map: [Bitboard; 4096],
}

/// Uses a slow, brute-force algorithm to generate all bishop and rook
/// magic numbers and returns them in a MagicManager struct.
pub fn gen_magic_numbers() -> MagicManager {
    let mut rooks: [Bitboard; 64] = [0; 64];
    let mut bishops: [Bitboard; 64] = [0; 64];
    for (i, _) in rooks.iter().enumerate() {
        let magic = gen_magic_number(i as u8);
        rooks[i] = magic;
    }
    for (i, _) in bishops.iter().enumerate() {
        let magic = gen_magic_number(i as u8);
        bishops[i] = magic;
    }
    MagicManager { rooks, bishops }
}

pub fn gen_magic_number(square: Square) -> u64 {
    let rook_mask = gen_rook_attacks_empty_board(A1 as u8);
    let mut all_possible_occupancies: [u64; 4096] = [0; 4096];
    for i in 0..4096 {
        all_possible_occupancies[i] = gen_occupancy(i, 12, rook_mask);
    }
    let mut attack_map: [u64; 4096] = [0; 4096];
    for i in 0..4096 {
        attack_map[i] = gen_legal_attacks(A1 as u8, all_possible_occupancies[i]);
    }

    let mut used_map: [u64; 4096] = [0; 4096];

    for k in 0..100000 {
        let magic = gen_random_number();
        let mut fail = false;
        for i in 0..4096 {
            used_map[i] = 0;
        }
        let mut i = 0;
        while i < 4096 && !fail {
            let occupied = all_possible_occupancies[i];
            let key = occupied.wrapping_mul(magic);
            let actual_key = (key >> (64 - 12)) as usize;
            if used_map[actual_key] == 0 {
                used_map[actual_key] = attack_map[i];
            } else if used_map[actual_key] != attack_map[i] {
                fail = true;
            }
            i += 1;
        }
        if !fail {
            return magic;
        }
    }
    0
}

/// Given an input square and a bitboard representing all blocking pieces on the board,
/// return a new Bitboard which represents all squares which are attacking candidates
/// for that piece.
///
/// In other words, this represents the "value" that will be returned when we lookup for a
/// certain magic number.
///
/// Note that we INCLUDE terminal blockers and outer squares in this value.
/// So, on the empty board with a rook on a1, this function would return a bitboard with non-zero
/// values on a1-a7 and a1-h1.
fn gen_legal_attacks(square: Square, blockers: Bitboard) -> Bitboard {
    let mut b: Bitboard = 0;
    let rank = square / 8;
    let file = square % 8;
    let rank_bb = rank_to_bb(rank);
    let file_bb = file_to_bb(file);
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

    let rank_bb = rank_to_bb(rank);
    let file_bb = file_to_bb(file);

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

/// Generates a random Bitboard (u64) with the goal of having sparse non-zero bits.
fn gen_random_number() -> u64 {
    let mut rng = rand::thread_rng();
    let n1: u64 = gen_u64();
    let n2: u64 = gen_u64();
    let n3: u64 = gen_u64();
    n1 & n2 & n3
}

fn gen_u64() -> u64 {
    let mut rng = rand::thread_rng();
    let u1: u64 = rng.gen();
    let u2: u64 = rng.gen();
    let u3: u64 = rng.gen();
    let u4: u64 = rng.gen();
    let u1 = u1 & 0xFFFF;
    let u2 = u2 & 0xFFFF;
    let u3 = u3 & 0xFFFF;
    let u4 = u4 & 0xFFFF;
    u1 | (u2 << 16) | (u3 << 32) | (u4 << 48)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::square::SquareIndex::{A1, A8, B2, D4, H1, H8};

    #[test]
    fn correct_attack_board_a1() {
        let b = gen_rook_attacks_empty_board(A1 as u8);
        assert_eq!(b, 282578800148862);
    }

    #[test]
    fn correct_attack_board_b2() {
        let b = gen_rook_attacks_empty_board(B2 as u8);
        assert_eq!(b, 565157600328704);
    }

    #[test]
    fn correct_attack_board_d4() {
        let b = gen_rook_attacks_empty_board(D4 as u8);
        assert_eq!(b, 2260632246683648);
    }

    #[test]
    fn correct_attack_board_h1() {
        let b = gen_rook_attacks_empty_board(H1 as u8);
        assert_eq!(b, 36170086419038334);
    }

    #[test]
    fn correct_attack_board_h8() {
        let b = gen_rook_attacks_empty_board(H8 as u8);
        assert_eq!(b, 9115426935197958144);
    }

    #[test]
    fn correct_attack_board_a8() {
        let b = gen_rook_attacks_empty_board(A8 as u8);
        assert_eq!(b, 9079539427579068672);
    }

    #[test]
    fn correct_legal_attacks_d4() {
        let blockers: Bitboard = 576460752303423488;
        let b = gen_legal_attacks(D4 as u8, blockers);
        assert_eq!(b, 578721386714368008);

        let blockers: Bitboard = 8797200320512;
        let b = gen_legal_attacks(D4 as u8, blockers);
        assert_eq!(b, 8832432998400);
    }

    #[test]
    fn correct_combinations() {
        let b = gen_magic_number(0);
        assert_eq!(b, 756607761056301088);
    }
}
