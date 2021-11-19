use std::mem;

use crate::common::{chess_move::Move, eval_move::EvaledMove};
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

    /// Saves the given entry into the table, returns whether
    /// or not the entry could be successfully added to the transposition table.
    /// Uses an always-replace strategy to resolve collisions.
    pub fn save(&mut self, hash: u64, entry: Entry) -> bool {
        let index = hash as usize % self.table.len();
        self.table[index] = Some(entry);
        true
    }

    /// Using the given hash, return the Entry which is associated with it in the table.
    pub fn get(&self, hash: u64) -> Option<Entry> {
        let index = hash as usize % self.table.len();
        self.table[index]
    }
}
