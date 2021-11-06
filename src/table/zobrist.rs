use crate::common::piece::PIECE_COUNT;
use rand::prelude::ThreadRng;
use rand::RngCore;

type ZobristNumber = u64;
type ZobristBoard = [ZobristNumber; 64];

/// A ZobristTable maintains the random values needed to create Zobrist hashes
/// for use in a transposition table.
pub struct ZobristTable {
    pub white_table: [ZobristBoard; PIECE_COUNT],
    pub black_table: [ZobristBoard; PIECE_COUNT],
    pub whites_turn: ZobristNumber,
    pub castling_rights: [ZobristNumber; 4],
    pub en_passant_file: [ZobristNumber; 8],
}

impl ZobristTable {
    pub fn init() -> ZobristTable {
        let mut rng = rand::thread_rng();

        let white_table = ZobristTable::gen_pieces(&mut rng);
        let black_table = ZobristTable::gen_pieces(&mut rng);
        let whites_turn = rng.next_u64();
        let castling_rights = ZobristTable::gen_castling(&mut rng);
        let en_passant_file = ZobristTable::gen_enpassant(&mut rng);

        ZobristTable {
            white_table,
            black_table,
            whites_turn,
            castling_rights,
            en_passant_file
        }
    }

    fn gen_pieces(rng: &mut ThreadRng) -> [ZobristBoard; PIECE_COUNT] {
        let mut table  = [[0u64; 64]; PIECE_COUNT];
        for i in 0..PIECE_COUNT {
            let mut piece = table[i];
            for j in 0..64 {
                piece[j] = rng.next_u64();
            }
        }

        table
    }

    fn gen_castling(rng: &mut ThreadRng) -> [ZobristNumber; 4] {
        let mut table = [0u64; 4];
        for i in 0..4 {
            table[i] = rng.next_u64();
        }
        table
    }

    fn gen_enpassant(rng: &mut ThreadRng) -> [ZobristNumber; 8] {
        let mut table = [0u64; 8];
        for i in 0..8 {
            table[i] = rng.next_u64();
        }
        table
    }
}
