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

static MAXIMUM_ITERATIONS: usize = 1000000;

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
            let magic =
                compute_magic(i as u8, piece, random, &mut table[start_index..end_index]).unwrap();
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

    /// Given a MagicPiece, initialize an array which represents the offsets that are needed to
    /// calculate indices for a contiguous array of all magic number lookups.
    ///
    /// As an example, the square A1 has no required offset. However, if the square A1 contained
    /// 12 relevant bits then the square A2 would have an offset of 4096 (i.e. 2^12).
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

    /// Calculate the total size of a contiguous array needed to store all magic number lookups.
    fn calc_len(piece: MagicPiece) -> usize {
        match piece {
            MagicPiece::Rook => ROOK_RELEVANT_BITS.iter().map(|x| 1 << *x).sum(),
            MagicPiece::Bishop => BISHOP_RELEVANT_BITS.iter().map(|x| 1 << *x).sum(),
        }
    }

    /// Initialize the ray that is used to mask relevant bits. This resulting Bitboard contains
    /// neither the provided square nor the border squares on the board which are the end of
    /// attacking rays.
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
    /// valid captures). Note that the provided square is not included in the returned bitboard.
    pub fn moves(&self, square: Square, blockers: Bitboard) -> Bitboard {
        let mask = self.rays[square as usize];
        let occupancy = mask & blockers;
        let bits = match self.piece {
            MagicPiece::Rook => ROOK_RELEVANT_BITS[square as usize],
            MagicPiece::Bishop => BISHOP_RELEVANT_BITS[square as usize],
        };
        let magic = self.magics[square as usize];
        let offset = self.offset[square as usize];

        let key = key(occupancy, magic, bits);

        self.table[offset + key]
    }
}

pub fn compute_magic(
    square: Square,
    piece: MagicPiece,
    random: &mut impl Random,
    table: &mut [u64],
) -> Option<u64> {
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

    search_magic(random, occupancies, attack_map, bits, table)
}

pub fn key(occupied: Bitboard, magic: u64, bits: usize) -> usize {
    (occupied.wrapping_mul(magic) >> (64 - bits)) as usize
}

fn search_magic(
    random: &mut impl Random,
    occupancies: [Bitboard; 4096],
    attack_map: [Bitboard; 4096],
    bits: usize,
    table: &mut [u64],
) -> Option<u64> {
    for k in 0..MAXIMUM_ITERATIONS {
        let magic = random.gen_random_number();
        let mut fail = false;
        table.iter_mut().for_each(|m| *m = 0);
        'inner: for i in 0..(1 << bits) {
            let occupied = occupancies[i];
            let key = key(occupied, magic, bits);
            if table[key] == 0 {
                table[key] = attack_map[i];
            } else if table[key] != attack_map[i] {
                fail = true;
                break 'inner;
            }
        }
        if !fail {
            println!("it took {} iterations", k);
            return Some(magic);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::square::SquareIndex::{A1, A8, B2, C7, D4, H1, H8};

    #[test]
    fn correct_combinations() {}
}
