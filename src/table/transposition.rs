use std::mem;

use crate::common::{chess_move::Move, eval_move::EvaledMove};
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Entry {
    pub score: i32,
    pub best_move: EvaledMove,
    pub hash: u64,
    pub depth: u8,
}

pub struct TranspositionTable {
    table: Vec<Option<Entry>>,
}

/// A transpositon table is a lightweight hash map which maps Zobrist hashes (u64s) to entries.
/// The table uses an "always-replace" replacement strategy, meaning that the most recent entry to occupy
/// an index will be the one stored there.
impl TranspositionTable {
    /// Constructs a new TranspositionTable with the given number of entries
    pub fn new(size: usize) -> TranspositionTable {
        TranspositionTable {
            table: vec![None; size],
        }
    }

    /// Constructs a new TranspositionTable with the given number of entries
    pub fn new_mb(size: usize) -> TranspositionTable {
        let size = size * 1024 * 1024 / mem::size_of::<Entry>();
        TranspositionTable {
            table: vec![None; size],
        }
    }

    /// Saves the given entry into the table, returns whether
    /// or not the entry could be successfully added to the transposition table.
    pub fn save(&mut self, hash: u64, entry: Entry, depth: usize) -> bool {
        let index = hash as usize % self.table.len();
        let current_entry = self.table[index];

        self.table[index] = Some(entry);
        true
    }

    /// Using the given hash, return the Entry which is associated with it in the table.
    pub fn get(&self, hash: u64, depth: usize) -> Option<Entry> {
        let index = hash as usize % self.table.len();
        let entry = self.table[index];
        match entry {
            None => None,
            Some(e) => {
                if e.depth < depth as u8 && e.hash == hash {
                    return Some(e)
                }
                None
            }
        }
    }
}
