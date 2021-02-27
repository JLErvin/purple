pub type Bitboard = u64;

pub const RANK1: u64 = 0xFF;
pub const RANK2: u64 = RANK1 << (8 * 1);
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

#[cfg(test)]
mod tests {
    use super::*;

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
