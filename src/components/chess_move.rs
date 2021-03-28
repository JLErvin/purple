use crate::components::piece::Piece;

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
