use crate::components::bitboard::Bitboard;
use crate::components::square::Square;
use crate::magic::magic::{BISHOP_RELEVANT_BITS, ROOK_RELEVANT_BITS};
use crate::magic::random::Random;
use crate::magic::util::{
    bishop_attacks, bishop_ray, occupancy, rook_attacks, rook_ray, MagicPiece,
};

static MAXIMUM_ITERATIONS: usize = 1000000;

struct MagicSearcher {
    occupancies: [Bitboard; 4096],
    attack_map: [Bitboard; 4096],
    bits: usize,
}

impl MagicSearcher {
    fn new(square: Square, piece: MagicPiece) -> MagicSearcher {
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
        MagicSearcher {
            occupancies,
            attack_map,
            bits,
        }
    }
}

pub fn compute_magic(
    square: Square,
    piece: MagicPiece,
    random: &mut impl Random,
    table: &mut [u64],
) -> Option<u64> {
    let searcher = MagicSearcher::new(square, piece);

    search_magic(random, searcher, table)
}

pub fn key(occupied: Bitboard, magic: u64, bits: usize) -> usize {
    (occupied.wrapping_mul(magic) >> (64 - bits)) as usize
}

fn search_magic(
    random: &mut impl Random,
    searcher: MagicSearcher,
    table: &mut [u64],
) -> Option<u64> {
    for k in 0..MAXIMUM_ITERATIONS {
        let magic = random.gen_random_number();
        table.iter_mut().for_each(|m| *m = 0);
        let passed = validate_magic(magic, &searcher, table);
        if passed {
            return Some(magic);
        }
    }
    None
}

#[inline]
fn validate_magic(magic: u64, searcher: &MagicSearcher, table: &mut [u64]) -> bool {
    for i in 0..table.len() {
        let occupied = searcher.occupancies[i];
        let key = key(occupied, magic, searcher.bits);
        if table[key] == 0 {
            table[key] = searcher.attack_map[i];
        } else if table[key] != searcher.attack_map[i] {
            return false;
        }
    }
    true
}
