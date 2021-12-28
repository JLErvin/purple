use crate::board_state::board::BoardState;
use crate::common::bitboard::{Bitboard, PieceItr};
use crate::common::piece::{Color, PieceType};
use crate::common::square::square_to_file;
use itertools::Itertools;
use rand::prelude::ThreadRng;
use rand::RngCore;

type ZobristHash = u64;

/// A ZobristTable maintains the random values needed to create Zobrist hashes
/// for use in a transposition table.
pub struct ZobristTable {
    pub table: [u64; 2 * 6 * 64],
    pub whites_turn: ZobristHash,
    pub castling_rights: [ZobristHash; 4],
    pub en_passant_file: [ZobristHash; 8],
}

/// A ZobristTable manages the randomly generated ZobristHashes for a given session
impl ZobristTable {
    pub fn init() -> ZobristTable {
        let mut rng = rand::thread_rng();

        let len = 2 * 6 * 64;
        let mut table: [u64; 2 * 64 * 6] = [0; 2 * 6 * 64];
        for i in 0..len {
            table[i] = rng.next_u64();
        }

        let whites_turn = rng.next_u64();
        let castling_rights = ZobristTable::gen_castling(&mut rng);
        let en_passant_file = ZobristTable::gen_enpassant(&mut rng);

        ZobristTable {
            table,
            whites_turn,
            castling_rights,
            en_passant_file,
        }
    }

    fn gen_castling(rng: &mut ThreadRng) -> [ZobristHash; 4] {
        let mut table = [0u64; 4];
        for i in 0..4 {
            table[i] = rng.next_u64();
        }
        table
    }

    fn gen_enpassant(rng: &mut ThreadRng) -> [ZobristHash; 8] {
        let mut table = [0u64; 8];
        for i in 0..8 {
            table[i] = rng.next_u64();
        }
        table
    }

    pub fn hash(&self, pos: &mut BoardState) -> ZobristHash {
        let mut hash: ZobristHash = 0;
        for (piece, color) in PieceType::iterator().cartesian_product(Color::iterator()) {
            let bb: Bitboard = pos.bb(*color, *piece);
            let i = match *piece {
                PieceType::Pawn => 0,
                PieceType::Rook => 1,
                PieceType::Knight => 2,
                PieceType::Bishop => 3,
                PieceType::Queen => 4,
                PieceType::King => 5,
            };
            for (j, _) in bb.iter() {
                let index = match color {
                    Color::White => (i * 64) + j as usize,
                    Color::Black => (i * 64) + j as usize + 384 as usize,
                };
                hash ^= self.table[index];
            }
        }

        if pos.castling_rights().black_king {
            hash ^= self.castling_rights[0];
        }

        if pos.castling_rights().black_queen {
            hash ^= self.castling_rights[1];
        }

        if pos.castling_rights().white_king {
            hash ^= self.castling_rights[2];
        }

        if pos.castling_rights().white_queen {
            hash ^= self.castling_rights[3];
        }

        match pos.en_passant() {
            None => (),
            Some(e) => hash ^= self.en_passant_file[square_to_file(e) as usize],
        };

        if pos.active_player() == Color::White {
            hash ^= self.whites_turn;
        }

        hash
    }
}

#[cfg(test)]
mod test {
    use crate::board_state::fen::parse_fen;

    use super::ZobristTable;

    #[test]
    fn same_position_should_have_same_hash() {
        let zobrist = ZobristTable::init();

        let mut pos1 = parse_fen(&"k7/8/2K5/8/8/8/8/1Q6 w - - 0 1".to_string()).unwrap();
        let mut pos2 = parse_fen(&"k7/8/2K5/8/8/8/8/1Q6 w - - 0 1".to_string()).unwrap();

        let hash1 = zobrist.hash(&mut pos1);
        let hash2 = zobrist.hash(&mut pos2);

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn different_positions_should_have_different_hashes() {
        let zobrist = ZobristTable::init();

        let mut pos1 = parse_fen(&"k7/8/2K5/8/8/8/8/1Q6 w - - 0 1".to_string()).unwrap();
        let mut pos2 =
            parse_fen(&"r2qkbnr/ppp2ppp/2np4/8/8/PPPpPbP1/7P/RNBQKBNR w KQkq - 0 8".to_string())
                .unwrap();

        let hash1 = zobrist.hash(&mut pos1);
        let hash2 = zobrist.hash(&mut pos2);

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn should_differentiate_between_players() {
        let zobrist = ZobristTable::init();

        let mut pos1 = parse_fen(&"k7/8/2K5/8/8/8/8/1Q6 w - - 0 1".to_string()).unwrap();
        let mut pos2 = parse_fen(&"k7/8/2K5/8/8/8/8/1Q6 b - - 0 1".to_string()).unwrap();

        let hash1 = zobrist.hash(&mut pos1);
        let hash2 = zobrist.hash(&mut pos2);

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn different_positions_should_be_different() {
        let zobrist = ZobristTable::init();

        let mut pos1 = parse_fen(
            &"rnbqkbnr/1ppppppp/8/p7/3P4/1PN5/P1P1PPPP/R1BQKBNR b KQkq - 0 3".to_string(),
        )
        .unwrap();
        let mut pos2 =
            parse_fen(&"rnbqkbnr/1ppppppp/p7/8/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 1 2".to_string())
                .unwrap();

        let hash1 = zobrist.hash(&mut pos1);
        let hash2 = zobrist.hash(&mut pos2);

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn different_positions_should_be_different_2() {
        let zobrist = ZobristTable::init();

        let mut pos1 =
            parse_fen(&"rnbqkbnr/2pppppp/8/pp6/3P4/1PN5/PBP1PPPP/R2QKBNR b KQkq - 1 4".to_string())
                .unwrap();
        let mut pos2 = parse_fen(
            &"rnbqkbnr/1ppppppp/8/p7/3P4/1PN5/P1P1PPPP/R1BQKBNR b KQkq - 0 3".to_string(),
        )
        .unwrap();

        let hash1 = zobrist.hash(&mut pos1);
        let hash2 = zobrist.hash(&mut pos2);

        assert_ne!(hash1, hash2);
    }
}
