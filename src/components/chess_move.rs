use crate::components::chess_move::MoveType::{
    BishopPromotion, BishopPromotionCapture, KnightPromotion, KnightPromotionCapture,
    QueenPromotion, QueenPromotionCapture, RookPromotion, RookPromotionCapture,
};
use crate::components::piece::PieceType::Knight;
use crate::components::piece::{Piece, PieceType};
use crate::components::square::algebraic_to_square;
use std::slice::Iter;

pub const NORTH: i8 = 8;
pub const EAST: i8 = 1;
pub const SOUTH: i8 = -NORTH;
pub const WEST: i8 = -EAST;

#[derive(Copy, Clone)]
pub struct Move {
    pub to: u8,
    pub from: u8,
    pub kind: MoveType,
}

#[derive(Copy, Clone, PartialEq)]
pub enum MoveType {
    Capture,
    EnPassantCapture,
    KnightPromotion,
    BishopPromotion,
    RookPromotion,
    QueenPromotion,
    KnightPromotionCapture,
    BishopPromotionCapture,
    RookPromotionCapture,
    QueenPromotionCapture,
    Quiet,
    CastleKing,
    CastleQueen,
}

#[derive(Clone, Copy)]
pub enum PromotionType {
    Push,
    Capture,
}

fn rank_file_to_algebra(rank: u8, file: u8) -> String {
    let mut s: String = "".to_owned();
    let file = match file {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => "",
    };
    s.push_str(file);
    s.push_str(&*(rank + 1).to_string());
    s
}

impl Move {
    pub fn to_algebraic(&self) -> String {
        let to_rank = self.to / 8;
        let to_file = self.to % 8;

        let from_rank = self.from / 8;
        let from_file = self.from % 8;

        let mut s: String = "".to_owned();
        s.push_str(rank_file_to_algebra(from_rank, from_file).as_str());
        s.push_str(rank_file_to_algebra(to_rank, to_file).as_str());
        s
    }

    pub fn is_double_pawn_push(&self) -> bool {
        ((self.to as i8) - (self.from as i8)).abs() == 16
    }

    pub fn is_promotion_capture(&self) -> bool {
        self.kind == KnightPromotionCapture
            || self.kind == BishopPromotionCapture
            || self.kind == RookPromotionCapture
            || self.kind == QueenPromotionCapture
    }

    pub fn is_promotion(&self) -> bool {
        self.kind == KnightPromotion
            || self.kind == BishopPromotion
            || self.kind == RookPromotion
            || self.kind == QueenPromotion
    }

    pub fn is_castle(&self) -> bool {
        self.kind == MoveType::CastleKing || self.kind == MoveType::CastleQueen
    }

    pub fn promoted_piece(&self) -> Option<PieceType> {
        match self.kind {
            MoveType::RookPromotionCapture | MoveType::RookPromotion => Some(PieceType::Rook),
            MoveType::KnightPromotionCapture | MoveType::KnightPromotion => Some(PieceType::Knight),
            MoveType::BishopPromotionCapture | MoveType::BishopPromotion => Some(PieceType::Bishop),
            MoveType::QueenPromotionCapture | MoveType::QueenPromotion => Some(PieceType::Queen),
            _ => None,
        }
    }
}

impl MoveType {
    pub fn king_itr() -> Iter<'static, i8> {
        static KING_MOVES: [i8; 8] = [
            WEST,
            NORTH + WEST,
            NORTH,
            NORTH + EAST,
            EAST,
            SOUTH + EAST,
            SOUTH,
            SOUTH + WEST,
        ];
        KING_MOVES.iter()
    }

    pub fn promotion_itr() -> Iter<'static, MoveType> {
        static PROMOTIONS: [MoveType; 4] = [
            KnightPromotion,
            BishopPromotion,
            RookPromotion,
            QueenPromotion,
        ];
        PROMOTIONS.iter()
    }

    pub fn promotion_capture_itr() -> Iter<'static, MoveType> {
        static PROMOTIONS: [MoveType; 4] = [
            KnightPromotionCapture,
            BishopPromotionCapture,
            RookPromotionCapture,
            QueenPromotionCapture,
        ];
        PROMOTIONS.iter()
    }
}

#[cfg(test)]
mod test {
    use crate::components::chess_move::Move;
    use crate::components::chess_move::MoveType::Quiet;
    use crate::components::square::SquareIndex::{A2, A3};

    #[test]
    fn basic_move_to_long_algebra() {
        let m = Move {
            from: A2 as u8,
            to: A3 as u8,
            kind: Quiet,
        };
        let s = m.to_algebraic();
        assert_eq!(s, "a2a3");
    }
}
