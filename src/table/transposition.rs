use std::mem;

use crate::common::{chess_move::Move, eval_move::EvaledMove};
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Entry {
    pub best_move: EvaledMove,
    pub hash: u64,
    pub depth: u8,
    pub bound: Bound,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Bound {
    Upper,
    Lower,
    Exact,
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

    /// Constructs a new TranspositionTable with the given size in megabytes
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

        if current_entry.is_none() || current_entry.unwrap().depth <= depth as u8 {
            self.table[index] = Some(entry);
            return true;
        }

        false
    }

    /// Using the given hash, return the Entry which is associated with it in the table.
    pub fn get(&self, hash: u64, depth: usize) -> Option<Entry> {
        let index = hash as usize % self.table.len();
        self.table[index]
    }
}

/*
function AlphaBetaWithMemory(n : node_type; alpha , beta , d : integer) : integer;
    if retrieve(n) == OK then /* Transposition table lookup */
        if n.lowerbound >= beta then return n.lowerbound;
        if n.upperbound <= alpha then return n.upperbound;
        alpha := max(alpha, n.lowerbound);
        beta := min(beta, n.upperbound);
    if d == 0 then g := evaluate(n); /* leaf node */
    else if n == MAXNODE then
        g := -INFINITY; a := alpha; /* save original alpha value */
        c := firstchild(n);
        while (g < beta) and (c != NOCHILD) do
            g := max(g, AlphaBetaWithMemory(c, a, beta, d - 1));
            a := max(a, g);
            c := nextbrother(c);
    else /* n is a MINNODE */
        g := +INFINITY; b := beta; /* save original beta value */
        c := firstchild(n);
        while (g > alpha) and (c != NOCHILD) do
            g := min(g, AlphaBetaWithMemory(c, alpha, b, d - 1));
            b := min(b, g);
            c := nextbrother(c);

    if g <= alpha then 
        n.upperbound := g; 
        store n.upperbound;
    if g >  alpha and g < beta then
        n.lowerbound := g; 
        n.upperbound := g; 
        store n.lowerbound, n.upperbound;
    if g >= beta then 
        n.lowerbound := g; 
        store n.lowerbound;
return g;
*/