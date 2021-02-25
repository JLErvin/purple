pub struct Board {
    w_pawns: u64,
    w_rooks: u64,
    w_knights: u64,
    w_bishops: u64,
    w_king: u64,
    w_queen: u64,

    b_pawns: u64,
    b_rooks: u64,
    b_knights: u64,
    b_bishops: u64,
    b_king: u64,
    b_queen: u64,
}

impl Board {
    pub fn default() -> Board {
        Board {
            w_pawns: 65280,
            w_rooks: 129,
            w_knights: 66,
            w_bishops: 36,
            w_queen: 16,
            w_king: 8,
            b_pawns: 71776119061217280,
            b_rooks: 9295429630892703744,
            b_knights: 4755801206503243776,
            b_bishops: 2594073385365405696,
            b_queen: 1152921504606846976,
            b_king: 576460752303423488,
        }
    }

    pub fn pieces(&self) -> Vec<u64> {
        vec![
            self.w_pawns,
            self.w_rooks,
            self.w_knights,
            self.w_bishops,
            self.w_king,
            self.w_queen,
            self.b_pawns,
            self.b_rooks,
            self.b_knights,
            self.b_bishops,
            self.b_king,
            self.b_queen,
        ]
    }

    pub fn debug_print(&self) {
        let mut p: u64 = 0;
        for b in self.pieces().iter() {
            p |= b
        }
        for i in 1..65 {
            print!("{}", Board::bit_at_i(p, i - 1) as u64);
            if i % 8 == 0 {
                println!("");
            }
        }
        println!("");
    }

    pub fn bit_at_i(n: u64, i: u8) -> bool {
        n & (1 << i) != 0
    }
}
