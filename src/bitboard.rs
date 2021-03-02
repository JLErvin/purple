use crate::square::*;

pub type Bitboard = u64;

pub const RANK1: u64 = 0xFF;
pub const RANK2: u64 = RANK1 << 8;
pub const RANK3: u64 = RANK1 << (8 * 2);
pub const RANK4: u64 = RANK1 << (8 * 3);
pub const RANK5: u64 = RANK1 << (8 * 4);
pub const RANK6: u64 = RANK1 << (8 * 5);
pub const RANK7: u64 = RANK1 << (8 * 6);
pub const RANK8: u64 = RANK1 << (8 * 7);

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
    fn identifies_index_top_left() {
        let index = rank_file_to_index(8, 1);
        assert_eq!(index, 63);
    }

    #[test]
    fn identifies_index_top_right() {
        let index = rank_file_to_index(8, 8);
        assert_eq!(index, 56);
    }

    #[test]
    fn identifies_index_bottom_left() {
        let index = rank_file_to_index(1, 1);
        assert_eq!(index, 7);
    }

    #[test]
    fn identifies_index_bottom_right() {
        let index = rank_file_to_index(1, 8);
        assert_eq!(index, 0);
    }

    #[test]
    fn identifies_index_middle() {
        let index = rank_file_to_index(4, 5);
        assert_eq!(index, 27);
    }

    #[test]
    fn adds_piece_eight_rank() {
        let b1: Bitboard = 0;
        let b2 = b1.add_piece(8, 1);
        assert_eq!(b2, 1 << 63);
    }

    #[test]
    fn adds_piece_eight_rank_and_file() {
        let b1: Bitboard = 0;
        let b2 = b1.add_piece(8, 8);
        assert_eq!(b2, 1 << 56);
    }

    #[test]
    fn adds_piece_first_rank() {
        let b1: Bitboard = 0;
        let b2 = b1.add_piece(1, 1);
        assert_eq!(b2, 1 << 7);
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
