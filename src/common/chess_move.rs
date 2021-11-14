use crate::common::chess_move::MoveType::{
    BishopPromotion, BishopPromotionCapture, KnightPromotion, KnightPromotionCapture, Null,
    QueenPromotion, QueenPromotionCapture, RookPromotion, RookPromotionCapture,
};

use crate::common::piece::PieceType;

use std::slice::Iter;

pub const NORTH: i8 = 8;
pub const EAST: i8 = 1;
pub const SOUTH: i8 = -NORTH;
pub const WEST: i8 = -EAST;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Move {
    pub to: u8,
    pub from: u8,
    pub kind: MoveType,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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
    Null,
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
    pub fn null() -> Move {
        Move {
            to: 0,
            from: 0,
            kind: Null,
        }
    }

    pub fn to_algebraic(&self) -> String {
        let to_rank = self.to / 8;
        let to_file = self.to % 8;

        let from_rank = self.from / 8;
        let from_file = self.from % 8;

        let mut s: String = "".to_owned();
        s.push_str(rank_file_to_algebra(from_rank, from_file).as_str());
        s.push_str(rank_file_to_algebra(to_rank, to_file).as_str());

        if self.is_promotion() || self.is_promotion_capture() {
            match self.promoted_piece().unwrap() {
                PieceType::Rook => s.push('r'),
                PieceType::Knight => s.push('n'),
                PieceType::Bishop => s.push('b'),
                PieceType::Queen => s.push('q'),
                _ => {}
            }
        }

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

    pub fn is_capture(&self) -> bool {
        self.kind == MoveType::Capture || self.kind == MoveType::EnPassantCapture || self.is_promotion_capture()
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
    use crate::common::chess_move::Move;
    use crate::common::chess_move::MoveType::Quiet;
    use crate::common::square::SquareIndex::{A2, A3};

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
