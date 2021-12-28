use std::mem;

use crate::common::eval_move::EvaledMove;
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
}

#[cfg(test)]
mod test {
    use crate::common::eval_move::EvaledMove;
    use crate::table::transposition::{Bound, Entry, TranspositionTable};

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
    fn should__not_replace_entry_with_shallower_depth() {
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
