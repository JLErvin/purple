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
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn convert_char_to_piece(c: char) -> Piece {
        let color = match c.is_lowercase() {
            true => Color::Black,
            false => Color::White,
        };
        let piece_type = match c.to_ascii_lowercase() {
            'p' => PieceType::Pawn,
            'r' => PieceType::Rook,
            'n' => PieceType::Knight,
            'b' => PieceType::Bishop,
            'k' => PieceType::King,
            'q' => PieceType::Queen,
            _ => PieceType::Pawn,
        };
        Piece { piece_type, color }
    }
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
