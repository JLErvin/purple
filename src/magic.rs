mod constants;

use std::slice::Iter;

use rand::prelude::ThreadRng;
use rand::RngCore;

use self::constants::{BISHOP_RELEVANT_BITS, ROOK_RELEVANT_BITS};
use crate::bitboard::{
    AddPiece, Bitboard, ClearBit, GetBit, FILEA, FILEB, FILEC, FILED, FILEE, FILEF, FILEG, FILEH,
    RANK1, RANK2, RANK3, RANK4, RANK5, RANK6, RANK7, RANK8,
};
use crate::magic::constants::MAGIC_NUMBERS;
use crate::square::{rank_file_to_index, Square};

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
        for i in 1..offset.len() {
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
    for _ in 0..MAXIMUM_ITERATIONS {
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

#[derive(PartialEq)]
pub enum GenerationScheme {
    #[allow(dead_code)]
    PseudoRandom, // Used if magic numbers should be calculated on startup
    PreComputed,
}

pub struct MagicRandomizer {
    random: ThreadRng,
    scheme: GenerationScheme,
    itr: Box<Iter<'static, u64>>,
}

pub trait Random {
    fn gen_random_number(&mut self) -> u64;
}

impl Random for MagicRandomizer {
    fn gen_random_number(&mut self) -> u64 {
        if self.scheme == GenerationScheme::PreComputed {
            return *self.itr.next().unwrap();
        }
        let n1: u64 = self.gen_u64();
        let n2: u64 = self.gen_u64();
        let n3: u64 = self.gen_u64();
        n1 & n2 & n3
    }
}

impl MagicRandomizer {
    pub fn new(scheme: GenerationScheme) -> MagicRandomizer {
        let itr = match scheme {
            GenerationScheme::PseudoRandom => Box::new([0; 0].iter()),
            GenerationScheme::PreComputed => Box::new(MAGIC_NUMBERS.iter()),
        };
        MagicRandomizer {
            random: ThreadRng::default(),
            scheme,
            itr,
        }
    }

    fn gen_u64(&mut self) -> u64 {
        let u1: u64 = self.random.next_u64() & 0xFFFF;
        let u2: u64 = self.random.next_u64() & 0xFFFF;
        let u3: u64 = self.random.next_u64() & 0xFFFF;
        let u4: u64 = self.random.next_u64() & 0xFFFF;
        u1 | (u2 << 16) | (u3 << 32) | (u4 << 48)
    }
}

#[derive(Copy, Clone)]
pub enum MagicPiece {
    Rook,
    Bishop,
}

/// Given the square for a rook and a bitboard representing all
/// blockers on the board, returns a new Bitboard which represents the potential
/// attacks that this rook might make based on the blocking pieces.
///
/// Note that this resulting Bitboard includes both border squares as well as blocking
/// pieces. For instance, a rook on A1 and a blocking piece on A6 would result
/// in a bitboard that covers A2,A3,A4,A5,A6 and B1,C2..,G7,H8
pub fn rook_attacks(square: Square, blockers: Bitboard) -> Bitboard {
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

/// Given the square for a rook generates a Bitboard which represents possible squares that the rook
/// might travel to. This resulting Bitboard excludes both the target square that the rook lives on
/// as well as border squares which are on the end of the rook's attacking ray.
/// For instance, a rook on A1 would return a Bitboard with occupancies on A2,..,A7 and B1,..G1.
pub fn rook_ray(square: Square) -> Bitboard {
    let mut square_bb: Bitboard = 0;
    square_bb = square_bb.add_at_square(square);
    let mut b: Bitboard;
    let rank = square / 8;
    let file = square % 8;

    let rank_bb = rank_to_bb(rank);
    let file_bb = file_to_bb(file);

    b = rank_bb | file_bb;
    b &= !square_bb;
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

/// Like the rook_ray function, except for bishops.
pub fn bishop_ray(square: Square) -> Bitboard {
    let mut b: Bitboard = 0;
    let rank = square / 8;
    let file = square % 8;

    let mut rank_itr = rank.saturating_add(1);
    let mut file_itr = file.saturating_add(1);
    while rank_itr < 8 && file_itr < 8 {
        b = b.add_piece(rank_itr, file_itr);
        rank_itr += 1;
        file_itr += 1;
    }

    let mut rank_itr = rank.saturating_add(1);
    let mut file_itr = file.saturating_sub(1);
    while rank_itr < 8 && file_itr > 0 {
        b = b.add_piece(rank_itr, file_itr);
        rank_itr += 1;
        file_itr -= 1;
    }

    let mut rank_itr = rank.saturating_sub(1);
    let mut file_itr = file.saturating_sub(1);
    while rank_itr > 0 && file_itr > 0 {
        b = b.add_piece(rank_itr, file_itr);
        rank_itr -= 1;
        file_itr -= 1;
    }

    let mut rank_itr = rank.saturating_sub(1);
    let mut file_itr = file.saturating_add(1);
    while rank_itr > 0 && file_itr < 8 {
        b = b.add_piece(rank_itr, file_itr);
        rank_itr -= 1;
        file_itr += 1;
    }

    let border = RANK1 | RANK8 | FILEA | FILEH;
    b & !border
}

/// Like the rook_attacks function, except for bishops.
pub fn bishop_attacks(square: Square, blockers: Bitboard) -> Bitboard {
    let ray = bishop_ray(square);
    let relevant_blockers = ray & blockers;
    let mut b: Bitboard = 0;
    let rank: i8 = (square / 8) as i8;
    let file: i8 = (square % 8) as i8;

    let mut rank_itr = rank.saturating_add(1);
    let mut file_itr = file.saturating_add(1);
    while rank_itr < 8 && file_itr < 8 {
        b = b.add_piece(rank_itr as u8, file_itr as u8);
        let s = rank_file_to_index(rank_itr as u8, file_itr as u8);
        if relevant_blockers.get_bit_lsb(s as i8) {
            break;
        }
        rank_itr += 1;
        file_itr += 1;
    }

    let mut rank_itr = rank.saturating_add(1);
    let mut file_itr = file.saturating_sub(1);
    while rank_itr < 8 && file_itr >= 0 {
        b = b.add_piece(rank_itr as u8, file_itr as u8);
        let s = rank_file_to_index(rank_itr as u8, file_itr as u8);
        if relevant_blockers.get_bit_lsb(s as i8) {
            break;
        }
        rank_itr += 1;
        file_itr -= 1;
    }

    let mut rank_itr = rank.saturating_sub(1);
    let mut file_itr = file.saturating_sub(1);
    while rank_itr >= 0 && file_itr >= 0 {
        b = b.add_piece(rank_itr as u8, file_itr as u8);
        let s = rank_file_to_index(rank_itr as u8, file_itr as u8);
        if relevant_blockers.get_bit_lsb(s as i8) {
            break;
        }
        rank_itr -= 1;
        file_itr -= 1;
    }

    let mut rank_itr = rank.saturating_sub(1);
    let mut file_itr = file.saturating_add(1);
    while rank_itr >= 0 && file_itr < 8 {
        b = b.add_piece(rank_itr as u8, file_itr as u8);
        let s = rank_file_to_index(rank_itr as u8, file_itr as u8);
        if relevant_blockers.get_bit_lsb(s as i8) {
            break;
        }
        rank_itr -= 1;
        file_itr += 1;
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

/// Given an occupancy index (some value between 0 and 1<<bits), the number of relevant bits,
/// and an attacking mask (i.e. bishop or rook ray), returns a new Bitboard which represents the relevant
/// occupancy position associated with that index.
///
/// This function should be used over all indices 0..1<<bits to generate all possible positions
/// of blocking pieces over a certain ray.
pub fn occupancy(occupancy_index: usize, bits: usize, mut attack_mask: Bitboard) -> Bitboard {
    let mut b: Bitboard = 0;

    for index in 0..bits {
        let square = attack_mask.trailing_zeros() as Bitboard;
        attack_mask = attack_mask.clear_bit(square as u8);
        if occupancy_index & (1 << index) != 0 {
            b |= 1 << square
        }
    }

    b
}
