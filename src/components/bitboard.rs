use crate::components::square::*;

pub type Bitboard = u64;

pub const RANK1: Bitboard = 0xFF;
pub const RANK2: Bitboard = RANK1 << 8;
pub const RANK3: Bitboard = RANK1 << (8 * 2);
pub const RANK4: Bitboard = RANK1 << (8 * 3);
pub const RANK5: Bitboard = RANK1 << (8 * 4);
pub const RANK6: Bitboard = RANK1 << (8 * 5);
pub const RANK7: Bitboard = RANK1 << (8 * 6);
pub const RANK8: Bitboard = RANK1 << (8 * 7);

pub const INIT_W_ROOKS: Bitboard = 0b_1000_0001_u64;
pub const INIT_W_KNIGHTS: Bitboard = 0b_0100_0010_u64;
pub const INIT_W_BISHOPS: Bitboard = 0b_0010_0100_u64;
pub const INIT_W_QUEEN: Bitboard = 0b_0001_0000_u64;
pub const INIT_W_KING: Bitboard = 0b_0000_1000_u64;

fn shift_left(n: u64, i: u8) -> Bitboard {
    n.checked_shl(i as u32).unwrap_or(0)
}

fn shift_right(n: u64, i: u8) -> Bitboard {
    n.checked_shr(i as u32).unwrap_or(0)
}

pub trait Shift {
    fn shift(&self, n: i8) -> Bitboard;
}

impl Shift for Bitboard {
    fn shift(&self, n: i8) -> Bitboard {
        if n > 0 {
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
}

impl AddPiece for Bitboard {
    fn add_piece(&self, rank: u8, file: u8) -> Bitboard {
        let index = rank_file_to_index(rank, file);
        *self | (1 << index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(b1 as u8, 1);
        assert_eq!(b2 as u8, 0);
    }

    #[test]
    fn gets_bit_msb() {
        let b: Bitboard = 0b0000_0001u64;
        let b1 = b.get_bit_msb(0);
        let b2 = b.get_bit_msb(63);
        assert_eq!(b1 as u8, 0);
        assert_eq!(b2 as u8, 1);
    }

    #[test]
    fn left_overflow_goes_to_zero() {
        let b: Bitboard = 0b0000_1000u64;
        let s = b.shift(64);
        assert_eq!(s, 0);
    }

    #[test]
    fn right_overflow_goes_to_zero() {
        let b: Bitboard = 0b0000_1000u64;
        let s = b.shift(-64);
        assert_eq!(s, 0);
    }

    #[test]
    fn shifts_left() {
        let b: Bitboard = 0b0000_1000u64;
        let s = b.shift(1);
        assert_eq!(s, 0b0001_0000u64);
    }

    #[test]
    fn shifts_right() {
        let b: Bitboard = 0b0000_1000;
        let s = b.shift(-1);
        assert_eq!(s, 0b0000_0100u64);
    }
}
