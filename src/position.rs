use crate::bitboard::*;

pub struct Position {
    w_pawns: Bitboard,
    w_rooks: Bitboard,
    w_knights: Bitboard,
    w_bishops: Bitboard,
    w_king: Bitboard,
    w_queen: Bitboard,

    b_pawns: Bitboard,
    b_rooks: Bitboard,
    b_knights: Bitboard,
    b_bishops: Bitboard,
    b_king: Bitboard,
    b_queen: Bitboard,

    w_turn: bool
}

impl Position {
    pub fn all(&self) -> Bitboard {
        self.our_all() | self.op_all()
    }

    pub fn op_no_king(&self) -> Bitboard {
        self.b_pawns | self.b_rooks | self.b_knights | self.b_bishops | self.b_queen
    }

    pub fn our_pawns(&self) -> Bitboard {
        if self.w_turn { self.w_pawns } else { self.b_pawns }
    }

    pub fn our_knights(&self) -> Bitboard {
        if self.w_turn { self.w_knights } else { self.b_knights }
    }

    pub fn our_bishops(&self) -> Bitboard {
        if self.w_turn { self.w_bishops } else { self.b_bishops }
    }

    pub fn our_king(&self) -> Bitboard {
        if self.w_turn { self.w_king } else { self.b_king }
    }

    pub fn our_queen(&self) -> Bitboard {
        if self.w_turn { self.w_queen } else { self.b_queen }
    }

    pub fn our_all(&self) -> Bitboard {
        if self.w_turn {
            self.w_pawns | self.w_rooks | self.w_knights | self.w_bishops | self.w_king | self.w_queen
        } else {
            self.b_pawns | self.b_rooks | self.b_knights | self.b_bishops | self.b_king | self.b_queen
        }

    }

    pub fn op_pawns(&self) -> Bitboard {
        if !self.w_turn { self.w_pawns } else { self.b_pawns }
    }

    pub fn op_knights(&self) -> Bitboard {
        if !self.w_turn { self.w_knights } else { self.b_knights }
    }

    pub fn op_bishops(&self) -> Bitboard {
        if !self.w_turn { self.w_bishops } else { self.b_bishops }
    }

    pub fn op_king(&self) -> Bitboard {
        if !self.w_turn { self.w_king } else { self.b_king }
    }

    pub fn op_queen(&self) -> Bitboard {
        if !self.w_turn { self.w_queen } else { self.b_queen }
    }

    pub fn op_all(&self) -> Bitboard {
        if !self.w_turn {
            self.w_pawns | self.w_rooks | self.w_knights | self.w_bishops | self.w_king | self.w_queen
        } else {
            self.b_pawns | self.b_rooks | self.b_knights | self.b_bishops | self.b_king | self.b_queen
        }

    }

    pub fn default() -> Position {
        Position {
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
            w_turn: true,
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
            print!("{}", p.get_bit_lsb(i-1) as u64);
            if i % 8 == 0 {
                println!("");
            }
        }
        println!("");
    }
}
