use crate::components::bitboard::{
    AddPiece, Bitboard, ClearBit, GetBit, FILEA, FILEB, FILEC, FILED, FILEE, FILEF, FILEG, FILEH,
    RANK1, RANK2, RANK3, RANK4, RANK5, RANK6, RANK7, RANK8,
};
use crate::components::square::{rank_file_to_index, Square};

#[derive(Copy, Clone)]
pub enum MagicPiece {
    Rook,
    Bishop,
}

/// Given the square for a rook and a bitboard representing all
/// blockers on the board, returns a new Bitboard which represents the potential
/// attacks that this rook might make based on the blocking pieces.
///
/// Note that this resulting Bitboard includes both border squares as well as blocking
/// pieces. For instance, a rook on A1 and a blocking piece on A6 would result
/// in a bitboard that covers A2,A3,A4,A5,A6 and B1,C2..,G7,H8
pub fn rook_attacks(square: Square, blockers: Bitboard) -> Bitboard {
    let mut b: Bitboard = 0;
    let rank = square / 8;
    let file = square % 8;
    let rank_bb = rank_to_bb(rank);
    let file_bb = file_to_bb(file);
    let relevant_blockers: Bitboard = (file_bb | rank_bb) & blockers;

    // in the NORTH direction
    for i in (rank + 1)..8 {
        b = b.add_piece(i, file);
        let s = rank_file_to_index(i, file);
        if relevant_blockers.get_bit_lsb(s as i8) {
            break;
        }
    }

    // in the EAST direction
    for i in (file + 1)..8 {
        b = b.add_piece(rank, i);
        let s = rank_file_to_index(rank, i);
        if relevant_blockers.get_bit_lsb(s as i8) {
            break;
        }
    }

    // in the SOUTH direction
    for i in (0..rank).rev() {
        b = b.add_piece(i, file);
        let s = rank_file_to_index(i, file);
        if relevant_blockers.get_bit_lsb(s as i8) {
            break;
        }
    }

    // in the WEST direction
    for i in (0..file).rev() {
        b = b.add_piece(rank, i);
        let s = rank_file_to_index(rank, i);
        if relevant_blockers.get_bit_lsb(s as i8) {
            break;
        }
    }

    b
}

/// Given the square for a rook generates a Bitboard which represents possible squares that the rook
/// might travel to. This resulting Bitboard excludes both the target square that the rook lives on
/// as well as border squares which are on the end of the rook's attacking ray.
/// For instance, a rook on A1 would return a Bitboard with occupancies on A2,..,A7 and B1,..G1.
pub fn rook_ray(square: Square) -> Bitboard {
    let mut square_bb: Bitboard = 0;
    square_bb = square_bb.add_at_square(square);
    let mut b: Bitboard = 0;
    let rank = square / 8;
    let file = square % 8;

    let rank_bb = rank_to_bb(rank);
    let file_bb = file_to_bb(file);

    b = rank_bb | file_bb;
    b = b & !square_bb;
    if file != 0 {
        b &= !FILEA;
    }
    if file != 7 {
        b &= !FILEH;
    }
    if rank != 0 {
        b &= !RANK1;
    }
    if rank != 7 {
        b &= !RANK8;
    }
    b
}

/// Like the rook_ray function, except for bishops.
pub fn bishop_ray(square: Square) -> Bitboard {
    let mut b: Bitboard = 0;
    let rank = square / 8;
    let file = square % 8;

    let mut rank_itr = rank.saturating_add(1);
    let mut file_itr = file.saturating_add(1);
    while rank_itr < 8 && file_itr < 8 {
        b = b.add_piece(rank_itr, file_itr);
        rank_itr += 1;
        file_itr += 1;
    }

    let mut rank_itr = rank.saturating_add(1);
    let mut file_itr = file.saturating_sub(1);
    while rank_itr < 8 && file_itr > 0 {
        b = b.add_piece(rank_itr, file_itr);
        rank_itr += 1;
        file_itr -= 1;
    }

    let mut rank_itr = rank.saturating_sub(1);
    let mut file_itr = file.saturating_sub(1);
    while rank_itr > 0 && file_itr > 0 {
        b = b.add_piece(rank_itr, file_itr);
        rank_itr -= 1;
        file_itr -= 1;
    }

    let mut rank_itr = rank.saturating_sub(1);
    let mut file_itr = file.saturating_add(1);
    while rank_itr > 0 && file_itr < 8 {
        b = b.add_piece(rank_itr, file_itr);
        rank_itr -= 1;
        file_itr += 1;
    }

    let border = RANK1 | RANK8 | FILEA | FILEH;
    b & !border
}

/// Like the rook_attacks function, except for bishops.
pub fn bishop_attacks(square: Square, blockers: Bitboard) -> Bitboard {
    let ray = bishop_ray(square);
    let relevant_blockers = ray & blockers;
    let mut b: Bitboard = 0;
    let rank = square / 8;
    let file = square % 8;

    let mut rank_itr = rank.saturating_add(1);
    let mut file_itr = file.saturating_add(1);
    while rank_itr < 8 && file_itr < 8 {
        b = b.add_piece(rank_itr, file_itr);
        let s = rank_file_to_index(rank_itr, file_itr);
        if relevant_blockers.get_bit_lsb(s as i8) {
            break;
        }
        rank_itr += 1;
        file_itr += 1;
    }

    let mut rank_itr = rank.saturating_add(1);
    let mut file_itr = file.saturating_sub(1);
    while rank_itr < 8 && file_itr > 0 {
        b = b.add_piece(rank_itr, file_itr);
        let s = rank_file_to_index(rank_itr, file_itr);
        if relevant_blockers.get_bit_lsb(s as i8) {
            break;
        }
        rank_itr += 1;
        file_itr -= 1;
    }

    let mut rank_itr = rank.saturating_sub(1);
    let mut file_itr = file.saturating_sub(1);
    while rank_itr > 0 && file_itr > 0 {
        b = b.add_piece(rank_itr, file_itr);
        let s = rank_file_to_index(rank_itr, file_itr);
        if relevant_blockers.get_bit_lsb(s as i8) {
            break;
        }
        rank_itr -= 1;
        file_itr -= 1;
    }

    let mut rank_itr = rank.saturating_sub(1);
    let mut file_itr = file.saturating_add(1);
    while rank_itr > 0 && file_itr < 8 {
        b = b.add_piece(rank_itr, file_itr);
        let s = rank_file_to_index(rank_itr, file_itr);
        if relevant_blockers.get_bit_lsb(s as i8) {
            break;
        }
        rank_itr -= 1;
        file_itr += 1;
    }

    b
}

fn rank_to_bb(rank: u8) -> Bitboard {
    match rank {
        0 => RANK1,
        1 => RANK2,
        2 => RANK3,
        3 => RANK4,
        4 => RANK5,
        5 => RANK6,
        6 => RANK7,
        7 => RANK8,
        _ => 0,
    }
}

fn file_to_bb(file: u8) -> Bitboard {
    match file {
        0 => FILEA,
        1 => FILEB,
        2 => FILEC,
        3 => FILED,
        4 => FILEE,
        5 => FILEF,
        6 => FILEG,
        7 => FILEH,
        _ => 0,
    }
}

/// Given an occupancy index (some value between 0 and 1<<bits), the number of relevant bits,
/// and an attacking mask (i.e. bishop or rook ray), returns a new Bitboard which represents the relevant
/// occupancy position associated with that index.
///
/// This function should be used over all indices 0..1<<bits to generate all possible positions
/// of blocking pieces over a certain ray.
pub fn occupancy(occupancy_index: usize, bits: usize, mut attack_mask: Bitboard) -> Bitboard {
    let mut b: Bitboard = 0;

    for index in 0..bits {
        let square = attack_mask.trailing_zeros() as Bitboard;
        attack_mask = attack_mask.clear_bit(square as u8);
        if occupancy_index & (1 << index) != 0 {
            b |= 1 << square
        }
    }

    b
}

#[cfg(test)]
mod tests {
    use crate::components::bitboard::Bitboard;
    use crate::components::square::SquareIndex::{A1, A8, B2, C7, D4, H1, H8};
    use crate::magic::util::{bishop_ray, rook_attacks, rook_ray};

    #[test]
    fn correct_rook_rays() {
        let a1 = rook_ray(A1 as u8);
        let b2 = rook_ray(B2 as u8);
        let d4 = rook_ray(D4 as u8);
        let h1 = rook_ray(H1 as u8);
        let h8 = rook_ray(H8 as u8);
        let a8 = rook_ray(A8 as u8);

        assert_eq!(a1, 282578800148862);
        assert_eq!(b2, 565157600328704);
        assert_eq!(d4, 2260632246683648);
        assert_eq!(h1, 36170086419038334);
        assert_eq!(h8, 9115426935197958144);
        assert_eq!(a8, 9079539427579068672);
    }

    #[test]
    fn correct_attacks_d4() {
        let blockers: Bitboard = 576460752303423488;
        let b = rook_attacks(D4 as u8, blockers);
        assert_eq!(b, 578721386714368008);

        let blockers: Bitboard = 8797200320512;
        let b = rook_attacks(D4 as u8, blockers);
        assert_eq!(b, 8832432998400);
    }

    #[test]
    fn correct_bishop_rays() {
        let a1 = bishop_ray(A1 as u8);
        let d4 = bishop_ray(D4 as u8);
        let c7 = bishop_ray(C7 as u8);

        assert_eq!(a1, 18049651735527936);
        assert_eq!(d4, 18051867805491712);
        assert_eq!(c7, 11064376819712);
    }
}
