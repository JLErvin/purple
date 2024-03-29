use crate::chess_move::{EAST, NORTH, SOUTH, WEST};
use crate::square::{rank_file_to_index, Square};

pub type Bitboard = u64;

pub const RANK1: Bitboard = 0xFF;
pub const RANK2: Bitboard = RANK1 << 8;
pub const RANK3: Bitboard = RANK1 << (8 * 2);
pub const RANK4: Bitboard = RANK1 << (8 * 3);
pub const RANK5: Bitboard = RANK1 << (8 * 4);
pub const RANK6: Bitboard = RANK1 << (8 * 5);
pub const RANK7: Bitboard = RANK1 << (8 * 6);
pub const RANK8: Bitboard = RANK1 << (8 * 7);

pub const FILEA: Bitboard =
    0b1_0000_0001_0000_0001_0000_0001_0000_0001_0000_0001_0000_0001_0000_0001_u64;
pub const FILEB: Bitboard = FILEA << 1;
pub const FILEC: Bitboard = FILEA << 2;
pub const FILED: Bitboard = FILEA << 3;
pub const FILEE: Bitboard = FILEA << 4;
pub const FILEF: Bitboard = FILEA << 5;
pub const FILEG: Bitboard = FILEA << 6;
pub const FILEH: Bitboard = FILEA << 7;

pub const INIT_W_ROOKS: Bitboard = 0b_1000_0001_u64;
pub const INIT_W_KNIGHTS: Bitboard = 0b_0100_0010_u64;
pub const INIT_W_BISHOPS: Bitboard = 0b_0010_0100_u64;
pub const INIT_W_QUEEN: Bitboard = 0b_0000_1000_u64;
pub const INIT_W_KING: Bitboard = 0b_0001_0000_u64;

fn shift_left(n: u64, i: u8) -> Bitboard {
    n.checked_shl(u32::from(i)).unwrap_or(0)
}

fn shift_right(n: u64, i: u8) -> Bitboard {
    n.checked_shr(u32::from(i)).unwrap_or(0)
}

pub trait Shift {
    fn shift(&self, n: i8) -> Bitboard;
}

impl Shift for Bitboard {
    fn shift(&self, n: i8) -> Bitboard {
        if n == NORTH {
            shift_left(*self, 8)
        } else if n == SOUTH {
            shift_right(*self, 8)
        } else if n == NORTH + NORTH {
            shift_left(*self, 16)
        } else if n == SOUTH + SOUTH {
            shift_right(*self, 16)
        } else if n == EAST {
            shift_left(*self & !FILEH, 1)
        } else if n == WEST {
            shift_right(*self & !FILEA, 1)
        } else if n == NORTH + EAST {
            shift_left(*self & !FILEH, 9)
        } else if n == NORTH + WEST {
            shift_left(*self & !FILEA, 7)
        } else if n == SOUTH + EAST {
            shift_right(*self & !FILEH, 7)
        } else if n == SOUTH + WEST {
            shift_right(*self & !FILEA, 9)
        } else if n > 0 {
            shift_left(*self, n as u8)
        } else {
            shift_right(*self, -n as u8)
        }
    }
}

pub trait GetBit {
    fn get_bit_lsb(&self, index: i8) -> bool;
    fn get_bit_msb(&self, index: i8) -> bool;
}

impl GetBit for Bitboard {
    fn get_bit_lsb(&self, index: i8) -> bool {
        self & (1 << index) != 0
    }
    fn get_bit_msb(&self, index: i8) -> bool {
        self & (1 << (63 - index)) != 0
    }
}

pub trait AddPiece {
    fn add_piece(&self, rank: u8, file: u8) -> Bitboard;
    fn add_at_square(&self, index: u8) -> Bitboard;
}

impl AddPiece for Bitboard {
    fn add_piece(&self, rank: u8, file: u8) -> Bitboard {
        let index = rank_file_to_index(rank, file);
        self.add_at_square(index)
    }

    fn add_at_square(&self, index: u8) -> Bitboard {
        *self | (1 << index)
    }
}

pub trait ClearBit {
    fn clear_bit(&self, index: u8) -> Bitboard;
}

impl ClearBit for Bitboard {
    fn clear_bit(&self, index: u8) -> Bitboard {
        *self & !(1 << index)
    }
}

pub struct BitboardIterator {
    bb: Bitboard,
}

impl Iterator for BitboardIterator {
    type Item = (Square, Bitboard);

    fn next(&mut self) -> Option<(Square, Bitboard)> {
        if self.bb == 0 {
            return None;
        }

        let square = self.bb.trailing_zeros() as u8;
        self.bb &= self.bb - 1;
        Some((square, self.bb))
    }
}

pub trait PieceItr {
    fn iter(&self) -> BitboardIterator;
}

impl PieceItr for Bitboard {
    fn iter(&self) -> BitboardIterator {
        BitboardIterator { bb: *self }
    }
}

pub trait New {
    fn empty() -> Bitboard;
    fn for_square(square: Square) -> Bitboard;
}
impl New for Bitboard {
    #[inline]
    fn empty() -> Bitboard {
        0
    }

    #[inline]
    fn for_square(square: Square) -> Bitboard {
        let b: Bitboard = 0;
        b.add_at_square(square)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess_move::{NORTH, WEST};

    #[test]
    fn adds_piece_eight_rank() {
        let b1: Bitboard = 0;
        let b2 = b1.add_piece(7, 0);
        assert_eq!(b2, 1 << 56);
    }

    #[test]
    fn adds_piece_eight_rank_and_file() {
        let b1: Bitboard = 0;
        let b2 = b1.add_piece(7, 7);
        assert_eq!(b2, 1 << 63);
    }

    #[test]
    fn adds_piece_first_rank() {
        let b1: Bitboard = 0;
        let b2 = b1.add_piece(0, 0);
        assert_eq!(b2, 1);
    }

    #[test]
    fn gets_bit_lsb() {
        let b: Bitboard = 0b0000_0001u64;
        let b1 = b.get_bit_lsb(0);
        let b2 = b.get_bit_lsb(63);
        assert_eq!(u8::from(b1), 1);
        assert_eq!(u8::from(b2), 0);
    }

    #[test]
    fn gets_bit_msb() {
        let b: Bitboard = 0b0000_0001u64;
        let b1 = b.get_bit_msb(0);
        let b2 = b.get_bit_msb(63);
        assert_eq!(u8::from(b1), 0);
        assert_eq!(u8::from(b2), 1);
    }

    #[test]
    fn left_overflow_goes_to_zero() {
        let b: Bitboard = 0b0000_1000u64;
        let s = b.shift(64);
        assert_eq!(s, 0);
    }

    #[test]
    fn left_overflow() {
        let b: Bitboard = 281_474_976_710_656;
        let s = b.shift(NORTH + WEST);
        assert_eq!(s, 0);
    }

    #[test]
    fn right_overflow_goes_to_zero() {
        let b: Bitboard = 0b0000_1000u64;
        let s = b.shift(-64);
        assert_eq!(s, 0);
    }
}
