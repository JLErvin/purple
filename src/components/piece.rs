use std::ops::{Index, Not};

#[derive(Copy, Clone)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
}

#[derive(Copy, Clone)]
pub enum Color {
    Black,
    White,
}

pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
