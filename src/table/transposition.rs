use std::{collections::HashSet, mem};

use crate::{
    board_state::board::BoardState,
    common::{chess_move::MoveType, eval_move::EvaledMove},
};

use super::zobrist::ZobristTable;
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

/*
            if !is_pv && t.depth() >= depth {
                match t.bound() {
                    Bound::Exact => {
                        return t.score();
                    },
                    Bound::Lower => {
                        if t.score() > alpha {
                            alpha = t.score();
                        }
                    },
                    Bound::Upper => {
                        if t.score() < beta {
                            beta = t.score();
                        }
                    }
                }
                if alpha >= beta {
                    return t.score();
                }
            }

            best_move = t.best_move();
*/
