use std::collections::HashSet;
use std::mem;

use itertools::Itertools;
use rand::prelude::ThreadRng;
use rand::RngCore;

use crate::bitboard::{Bitboard, PieceItr};
use crate::board::BoardState;
use crate::chess_move::{EvaledMove, MoveType};
use crate::piece::{Color, PieceType};
use crate::square::square_to_file;

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

        if pos.castling_rights.black_king {
            hash ^= self.castling_rights[0];
        }

        if pos.castling_rights.black_queen {
            hash ^= self.castling_rights[1];
        }

        if pos.castling_rights.white_king {
            hash ^= self.castling_rights[2];
        }

        if pos.castling_rights.white_queen {
            hash ^= self.castling_rights[3];
        }

        match pos.en_passant {
            None => (),
            Some(e) => hash ^= self.en_passant_file[square_to_file(e) as usize],
        };

        if pos.active_player == Color::White {
            hash ^= self.whites_turn;
        }

        hash
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Entry {
    pub best_move: EvaledMove,
    pub hash: u64,
    pub depth: u8,
    pub bound: Bound,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Bound {
    Upper,
    Lower,
    Exact,
}

/// A transposition table is a lightweight hash map which maps Zobrist hashes (u64s) to entries.
pub struct TranspositionTable {
    table: Vec<Option<Entry>>,
}

impl TranspositionTable {
    /// Constructs a new TranspositionTable with the given number of entries
    pub fn new(size: usize) -> TranspositionTable {
        TranspositionTable {
            table: vec![None; size],
        }
    }

    /// Constructs a new TranspositionTable with the given size in megabytes
    pub fn new_mb(size: usize) -> TranspositionTable {
        let size = size * 1024 * 1024 / mem::size_of::<Entry>();
        Self::new(size)
    }

    /// Saves the given entry into the table, returns whether or not the entry could be successfully saved.
    /// Replace entries if the currently saved entry has a depth less than or equal to
    /// the depth of the incoming entry.
    pub fn save(&mut self, hash: u64, entry: Entry) -> bool {
        let index = hash as usize % self.table.len();
        let curr_entry = self.table[index];
        if curr_entry.is_none() {
            self.table[index] = Some(entry);
            return true;
        }
        if let Some(curr_entry) = self.table[index] {
            if curr_entry.depth <= entry.depth {
                self.table[index] = Some(entry);
                return true;
            }
        }
        false
    }

    /// Using the given hash, return the Entry which is associated with it in the table.
    pub fn get(&self, hash: u64) -> Option<Entry> {
        let index = hash as usize % self.table.len();
        self.table[index]
    }

    /// Return the principal variation, starting with the given position
    #[allow(dead_code)]
    pub fn pv(&self, pos: &mut BoardState, zobrist: &ZobristTable) -> Vec<EvaledMove> {
        let mut pv = Vec::new();
        // Maintain a list of visited moves to avoid circular references in case of the PV being
        // a force-repetition
        let mut visited = HashSet::new();
        self.pv_inner(pos, &mut pv, &mut visited, zobrist);
        pv
    }

    pub fn pv_inner(
        &self,
        pos: &mut BoardState,
        pv: &mut Vec<EvaledMove>,
        visited: &mut HashSet<u64>,
        zobrist: &ZobristTable,
    ) {
        let hash = zobrist.hash(pos);
        let mv = self.get(hash);

        if let Some(m) = mv {
            if m.best_move.mv.kind == MoveType::Null {
                return;
            }
            pv.push(m.best_move);
            let mut new_pos = pos.clone_with_move(m.best_move.mv);

            if visited.insert(hash) {
                self.pv_inner(&mut new_pos, pv, visited, zobrist);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::chess_move::EvaledMove;
    use crate::fen::parse_fen;
    use crate::table::{Bound, Entry, TranspositionTable, ZobristTable};

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

    #[test]
    fn should_save_and_get_new_entry() {
        let mut table = TranspositionTable::new(10);
        let entry = Entry {
            best_move: EvaledMove::null(0),
            hash: 1,
            depth: 0,
            bound: Bound::Upper,
        };
        let was_saved = table.save(1, entry);
        assert_eq!(was_saved, true);
        let fetched_entry = table.get(1);
        assert_eq!(fetched_entry.is_some(), true);
        assert_eq!(fetched_entry.unwrap(), entry);
    }

    #[test]
    fn should_replace_entry_with_greater_depth() {
        let mut table = TranspositionTable::new(10);
        let entry_one = Entry {
            best_move: EvaledMove::null(0),
            hash: 1,
            depth: 0,
            bound: Bound::Upper,
        };
        let was_saved = table.save(1, entry_one);
        assert_eq!(was_saved, true);

        let entry_two = Entry {
            best_move: EvaledMove::null(0),
            hash: 1,
            depth: 10,
            bound: Bound::Upper,
        };
        let was_saved = table.save(1, entry_two);
        assert_eq!(was_saved, true);

        let fetched_entry = table.get(1);
        assert_eq!(fetched_entry.is_some(), true);
        assert_eq!(fetched_entry.unwrap(), entry_two);
    }

    #[test]
    fn should_not_replace_entry_with_shallower_depth() {
        let mut table = TranspositionTable::new(10);
        let entry_one = Entry {
            best_move: EvaledMove::null(0),
            hash: 1,
            depth: 10,
            bound: Bound::Upper,
        };
        let was_saved = table.save(1, entry_one);
        assert_eq!(was_saved, true);

        let entry_two = Entry {
            best_move: EvaledMove::null(0),
            hash: 1,
            depth: 1,
            bound: Bound::Upper,
        };
        let was_saved = table.save(1, entry_two);
        assert_eq!(was_saved, false);

        let fetched_entry = table.get(1);
        assert_eq!(fetched_entry.is_some(), true);
        assert_eq!(fetched_entry.unwrap(), entry_one);
    }
}
