use crate::components::bitboard::{
    AddPiece, Bitboard, ClearBit, New, Shift, FILEA, FILEB, FILEG, FILEH,
};
use crate::components::chess_move::{MoveType, EAST, NORTH, SOUTH, WEST};
use crate::components::piece::PieceType;
use crate::components::square::Square;
use crate::magic::magic::MagicTable;
use crate::magic::random::MagicRandomizer;
use crate::magic::util::{rook_ray, MagicPiece};
use crate::move_gen::util::knight_destinations;
use itertools::Itertools;

pub struct Lookup {
    rook_table: MagicTable,
    bishop_table: MagicTable,
    king_table: Vec<Bitboard>,
    knight_table: Vec<Bitboard>,
    between: [[Bitboard; 64]; 64],
}

impl Lookup {
    pub fn new(mut random: MagicRandomizer) -> Lookup {
        let rook_table = MagicTable::init(MagicPiece::Rook, &mut random);
        let bishop_table = MagicTable::init(MagicPiece::Bishop, &mut random);
        let king_table = Lookup::init_king();
        let knight_table = Lookup::init_knight();
        let between = Lookup::init_between(&rook_table, &bishop_table);

        Lookup {
            rook_table,
            bishop_table,
            king_table,
            knight_table,
            between,
        }
    }

    pub fn moves(&self, square: Square, piece: PieceType) -> Bitboard {
        match piece {
            PieceType::Knight => *self.knight_table.get(square as usize).unwrap(),
            PieceType::King => *self.king_table.get(square as usize).unwrap(),
            _ => 0,
        }
    }

    pub fn sliding_moves(&self, square: Square, blockers: Bitboard, piece: PieceType) -> Bitboard {
        match piece {
            PieceType::Rook => self.rook_table.moves(square, blockers),
            PieceType::Bishop => self.bishop_table.moves(square, blockers),
            PieceType::Queen => {
                self.rook_table.moves(square, blockers) | self.bishop_table.moves(square, blockers)
            }
            _ => 0,
        }
    }

    pub fn between(&self, s1: Square, s2: Square) -> Bitboard {
        self.between[s1 as usize][s2 as usize]
    }

    fn init_king() -> Vec<Bitboard> {
        let mut v: Vec<Bitboard> = Vec::with_capacity(64);

        for i in 0..64 {
            let mut b: Bitboard = 0;
            let mut r: Bitboard = 0;
            b = b.add_at_square(i);
            for dir in MoveType::king_itr() {
                r |= b.shift(*dir);
            }
            v.push(r);
        }

        v
    }

    fn init_knight() -> Vec<Bitboard> {
        let mut v: Vec<Bitboard> = Vec::with_capacity(64);

        for i in 0..64 {
            let b = knight_destinations(i as u8);
            v.push(b);
        }
        v
    }

    fn attacks(
        rook_table: &MagicTable,
        bishop_table: &MagicTable,
        square: Square,
        piece: MagicPiece,
    ) -> Bitboard {
        match piece {
            MagicPiece::Rook => rook_table.moves(square, 0),
            MagicPiece::Bishop => bishop_table.moves(square, 0),
        }
    }

    fn init_between(rook_table: &MagicTable, bishop_table: &MagicTable) -> [[Bitboard; 64]; 64] {
        let mut b: [[Bitboard; 64]; 64] = [[0; 64]; 64];

        for piece in vec![MagicPiece::Rook, MagicPiece::Bishop] {
            for (i, j) in (0..64).cartesian_product(0..64) {
                let bitboard_i = Bitboard::for_square(i);
                let bitboard_j = Bitboard::for_square(j);
                let attacks_i = Lookup::attacks(rook_table, bishop_table, i, piece);

                if attacks_i & bitboard_j != 0 {
                    match piece {
                        MagicPiece::Rook => {
                            b[i as usize][j as usize] =
                                attacks_i & rook_table.moves(j, 0) | bitboard_i | bitboard_j
                        }
                        MagicPiece::Bishop => {
                            b[i as usize][j as usize] =
                                attacks_i & bishop_table.moves(j, 0) | bitboard_i | bitboard_j
                        }
                    }
                }
            }
            /*            for i in 0..64 {
                            for j in 0..64 {
                                let k = Bitboard::for_square(j);
                                let p = Bitboard::for_square(i);
                                let attacks_i = match piece {
                                    MagicPiece::Rook => rook_table.moves(i, 0),
                                    MagicPiece::Bishop => bishop_table.moves(i, 0),
                                };
                                if k & attacks_i != 0 {
                                    match piece {
                                        MagicPiece::Rook => {
                                            b[i as usize][j as usize] =
                                                attacks_i & rook_table.moves(j, 0) | k | p
                                        }
                                        MagicPiece::Bishop => {
                                            b[i as usize][j as usize] =
                                                attacks_i & bishop_table.moves(j, 0) | k | p
                                        }
                                    }
                                }
                            }
                        }
            */
        }
        b
    }
}

#[cfg(test)]
mod test {
    use crate::components::square::SquareIndex::{A1, A8, D4, H1, H8};
    use crate::magic::random::*;
    use crate::move_gen::lookup::Lookup;

    #[test]
    fn init_between() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);

        let b = lookup.between(A1 as u8, H1 as u8);

        assert_eq!(b, 255);
    }

    #[test]
    fn init_king() {
        let t = Lookup::init_king();
        let a1 = *t.get(A1 as usize).unwrap();
        let h1 = *t.get(H1 as usize).unwrap();
        let a8 = *t.get(A8 as usize).unwrap();
        let h8 = *t.get(H8 as usize).unwrap();
        let d4 = *t.get(D4 as usize).unwrap();

        assert_eq!(a1, 770);
        assert_eq!(h1, 49216);
        assert_eq!(a8, 144959613005987840);
        assert_eq!(h8, 4665729213955833856);
        assert_eq!(d4, 120596463616);
    }

    #[test]
    fn init_knight() {
        let t = Lookup::init_knight();
        let a1 = *t.get(A1 as usize).unwrap();
        let h1 = *t.get(H1 as usize).unwrap();
        let a8 = *t.get(A8 as usize).unwrap();
        let h8 = *t.get(H8 as usize).unwrap();
        let d4 = *t.get(D4 as usize).unwrap();

        assert_eq!(a1, 132096);
        assert_eq!(h1, 4202496);
        assert_eq!(a8, 1128098930098176);
        assert_eq!(h8, 9077567998918656);
        assert_eq!(d4, 22136263676928);
    }

    #[test]
    fn test() {}
}
