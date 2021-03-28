use crate::components::chess_move::MoveType::{
    BishopPromotion, BishopPromotionCapture, KnightPromotion, KnightPromotionCapture,
    QueenPromotion, QueenPromotionCapture, RookPromotion, RookPromotionCapture,
};
use crate::components::piece::Piece;
use crate::components::piece::PieceType::Knight;
use std::slice::Iter;

pub const NORTH: i8 = 8;
pub const EAST: i8 = 1;
pub const SOUTH: i8 = -NORTH;
pub const WEST: i8 = -EAST;

pub struct Move {
    pub to: u8,
    pub from: u8,
    pub kind: MoveType,
}

#[derive(Copy, Clone)]
pub enum MoveType {
    Capture,
    KnightPromotion,
    BishopPromotion,
    RookPromotion,
    QueenPromotion,
    KnightPromotionCapture,
    BishopPromotionCapture,
    RookPromotionCapture,
    QueenPromotionCapture,
    Quiet,
}

impl MoveType {
    pub fn promotion_itr() -> Iter<'static, MoveType> {
        static DIRECTIONS: [MoveType; 4] = [
            KnightPromotion,
            BishopPromotion,
            RookPromotion,
            QueenPromotion,
        ];
        DIRECTIONS.iter()
    }

    pub fn promotion_capture_itr() -> Iter<'static, MoveType> {
        static DIRECTIONS: [MoveType; 4] = [
            KnightPromotionCapture,
            BishopPromotionCapture,
            RookPromotionCapture,
            QueenPromotionCapture,
        ];
        DIRECTIONS.iter()
    }
}
