use crate::components::bitboard::Bitboard;

use crate::components::square::Square;
use crate::magic::random::{MagicRandomizer, Random};

use crate::magic::util::{
    bishop_attacks, bishop_ray, occupancy, rook_attacks, rook_ray, MagicPiece,
};

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
    pub offset: [usize; 64],
    pub rays: [Bitboard; 64],
    pub piece: MagicPiece,
}

impl MagicTable {
    /// Initializes a new MagicTable object which holds magic numbers and the necessary infrastructure
    /// to lookup moves based on these values. Initialization of this struct is expensive as it
    /// involves finding all magic numbers for the given piece and builds the necessary lookup tables
    /// as well.
    pub fn init(piece: MagicPiece, random: &mut MagicRandomizer) -> MagicTable {
        let offset = MagicTable::init_offsets(piece);
        let len = MagicTable::calc_len(piece);
        let mut table = vec![0; len];
        let mut magics: Vec<u64> = Vec::with_capacity(64);
        let mut rays: [Bitboard; 64] = [0; 64];

        for i in 0..64 {
            let start_index = offset[i];
            rays[i] = MagicTable::init_ray(i as u8, piece);
            let end_index = match piece {
                MagicPiece::Rook => start_index + (1 << ROOK_RELEVANT_BITS[i]),
                MagicPiece::Bishop => start_index + (1 << BISHOP_RELEVANT_BITS[i]),
            };
            let mut m =
                MagicGenerator::new(i as u8, piece, random, &mut table[start_index..end_index]);
            let magic = m.find_magic_number();
            magics.push(magic);
        }

        MagicTable {
            table,
            magics,
            offset,
            rays,
            piece,
        }
    }

    fn init_offsets(piece: MagicPiece) -> [usize; 64] {
        let mut offset: [usize; 64] = [0; 64];
        for i in 1..64 {
            offset[i] = offset[i - 1]
                + match piece {
                    MagicPiece::Rook => 1 << ROOK_RELEVANT_BITS[i - 1],
                    MagicPiece::Bishop => 1 << BISHOP_RELEVANT_BITS[i - 1],
                }
        }
        offset
    }

    fn calc_len(piece: MagicPiece) -> usize {
        match piece {
            MagicPiece::Rook => ROOK_RELEVANT_BITS.iter().map(|x| 1 << *x).sum(),
            MagicPiece::Bishop => BISHOP_RELEVANT_BITS.iter().map(|x| 1 << *x).sum(),
        }
    }

    fn init_ray(square: Square, piece: MagicPiece) -> Bitboard {
        match piece {
            MagicPiece::Rook => rook_ray(square),
            MagicPiece::Bishop => bishop_ray(square),
        }
    }

    /// Returns a bitboard which represents the potential moves at the given square constrained
    /// by the given blockers bitboard. The blockers bitboard may include irrelevant pieces
    /// (i.e. those which are not on the ray associated with the given square) - this function
    /// will filter those out automatically.
    ///
    /// Note that the returned bitboard includes blocking and border-squares as those may represent
    /// valid captures (it is up to the client to determine whether or not those pieces represent
    /// valid captures).
    pub fn moves(&self, square: Square, blockers: Bitboard) -> Bitboard {
        let mask = self.rays[square as usize];
        let occupancy = mask & blockers;
        let bits = match self.piece {
            MagicPiece::Rook => ROOK_RELEVANT_BITS[square as usize],
            MagicPiece::Bishop => BISHOP_RELEVANT_BITS[square as usize],
        };
        let magic = self.magics[square as usize];
        let offset = self.offset[square as usize];

        let key = MagicGenerator::key(occupancy, magic, bits);

        self.table[offset + key]
    }
}

pub struct MagicGenerator<'a> {
    occupancies: [Bitboard; 4096],
    table: &'a mut [u64],
    attack_map: [Bitboard; 4096],
    bits: usize,
    random: &'a mut dyn Random,
}

impl MagicGenerator<'_> {
    /// Returns a new MagicGenerator for the given square and piece, using the given random
    /// object to generate potential random numbers. Magic numbers that are generated will be
    /// put into the provided table slice.
    pub fn new<'a>(
        square: Square,
        piece: MagicPiece,
        random: &'a mut impl Random,
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
            let magic = self.random.gen_random_number();
            let mut fail = false;
            self.table.iter_mut().for_each(|m| *m = 0);
            'inner: for i in 0..(1 << self.bits) {
                let occupied = self.occupancies[i];
                let key = MagicGenerator::key(occupied, magic, self.bits);
                if self.table[key] == 0 {
                    self.table[key] = self.attack_map[i];
                } else if self.table[key] != self.attack_map[i] {
                    fail = true;
                    break 'inner;
                }
            }
            if !fail {
                println!("it took {} iterations", k);
                return magic;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::square::SquareIndex::{A1, A8, B2, C7, D4, H1, H8};

    #[test]
    fn correct_combinations() {}
}
