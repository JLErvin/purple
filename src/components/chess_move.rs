use crate::components::chess_move::MoveType::{
    BishopPromotion, BishopPromotionCapture, KnightPromotion, KnightPromotionCapture,
    QueenPromotion, QueenPromotionCapture, RookPromotion, RookPromotionCapture,
};
use crate::components::piece::PieceType::Knight;
use crate::components::piece::{Piece, PieceType};
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

impl Move {
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
