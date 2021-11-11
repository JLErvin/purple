use crate::common::chess_move::Move;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Entry {
    pub score: i32,
    pub best_move: Move,
    pub hash16: u16,
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

    /// Saves the given entry into the table, returns whether
    /// or not the entry could be successfully added to the transposition table.
    pub fn save(&mut self, hash: u64, entry: Entry) -> bool {
        let index = hash as usize % self.table.len();
        let current_entry = self.table[index];

        match current_entry {
            Some(_) => false,
            None => {
                self.table[index] = Some(entry);
                true
            }
        }
    }

    /// Using the given hash, return the Entry which is associated with it in the table.
    pub fn get(&self, hash: u64) -> Option<Entry> {
        let index = hash as usize % self.table.len();
        self.table[index]
    }
}
