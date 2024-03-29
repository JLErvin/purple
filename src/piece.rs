use std::ops::{Index, IndexMut, Not};
use std::slice::Iter;

use crate::bitboard::Bitboard;

pub const PIECE_COUNT: usize = 6;
pub const COLOR_COUNT: usize = 2;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Color {
    Black,
    White,
}

pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Index<PieceType> for [Bitboard; PIECE_COUNT] {
    type Output = Bitboard;

    fn index(&self, piece: PieceType) -> &Self::Output {
        match piece {
            PieceType::Pawn => &self[0],
            PieceType::Rook => &self[1],
            PieceType::Knight => &self[2],
            PieceType::Bishop => &self[3],
            PieceType::Queen => &self[4],
            PieceType::King => &self[5],
        }
    }
}

impl IndexMut<PieceType> for [Bitboard; PIECE_COUNT] {
    fn index_mut(&mut self, piece: PieceType) -> &mut Self::Output {
        match piece {
            PieceType::Pawn => &mut self[0],
            PieceType::Rook => &mut self[1],
            PieceType::Knight => &mut self[2],
            PieceType::Bishop => &mut self[3],
            PieceType::Queen => &mut self[4],
            PieceType::King => &mut self[5],
        }
    }
}

impl Index<Color> for [Bitboard; COLOR_COUNT] {
    type Output = Bitboard;

    fn index(&self, color: Color) -> &Self::Output {
        match color {
            Color::White => &self[0],
            Color::Black => &self[1],
        }
    }
}

impl IndexMut<Color> for [Bitboard; COLOR_COUNT] {
    fn index_mut(&mut self, color: Color) -> &mut Self::Output {
        match color {
            Color::White => &mut self[0],
            Color::Black => &mut self[1],
        }
    }
}

impl Piece {
    pub fn convert_char_to_piece(c: char) -> PieceType {
        match c.to_ascii_lowercase() {
            'p' => PieceType::Pawn,
            'r' => PieceType::Rook,
            'n' => PieceType::Knight,
            'b' => PieceType::Bishop,
            'k' => PieceType::King,
            'q' => PieceType::Queen,
            _ => panic!("Cannot convert char to piece, unknown char: {}", c),
        }
    }

    pub fn convert_char_to_color(c: char) -> Color {
        if c.is_lowercase() {
            Color::Black
        } else {
            Color::White
        }
    }
}

impl PieceType {
    pub fn iterator() -> Iter<'static, PieceType> {
        static PIECES: [PieceType; 6] = [
            PieceType::Pawn,
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::King,
            PieceType::Queen,
        ];
        PIECES.iter()
    }

    pub fn value(&self) -> i64 {
        match self {
            PieceType::Pawn => 1,
            PieceType::Rook => 5,
            PieceType::Knight | &PieceType::Bishop => 3,
            PieceType::King => 0,
            PieceType::Queen => 8,
        }
    }

    pub fn idx(&self) -> usize {
        match self {
            PieceType::Pawn => 5,
            PieceType::Rook => 2,
            PieceType::Knight => 4,
            PieceType::Bishop => 3,
            PieceType::King => 0,
            PieceType::Queen => 1,
        }
    }
}

impl Color {
    pub fn iterator() -> Iter<'static, Color> {
        static COLORS: [Color; 2] = [Color::White, Color::Black];
        COLORS.iter()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_index_using_pieces() {
        let mut a: [Bitboard; PIECE_COUNT] = [0; PIECE_COUNT];
        a[0] = 1;
        a[1] = 2;
        a[2] = 3;
        a[3] = 5;
        a[4] = 7;
        a[5] = 11;
        assert_eq!(a[PieceType::Pawn], 1);
        assert_eq!(a[PieceType::Rook], 2);
        assert_eq!(a[PieceType::Knight], 3);
        assert_eq!(a[PieceType::Bishop], 5);
        assert_eq!(a[PieceType::Queen], 7);
        assert_eq!(a[PieceType::King], 11);
    }

    #[test]
    fn can_index_using_colors() {
        let mut a: [Bitboard; COLOR_COUNT] = [0; COLOR_COUNT];
        a[0] = 1;
        a[1] = 42;
        assert_eq!(a[Color::White], 1);
        assert_eq!(a[Color::Black], 42);
    }

    #[test]
    fn correct_not() {
        let white = Color::White;
        let black = Color::Black;
        assert_eq!(!white, Color::Black);
        assert_eq!(!black, Color::White);
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        Piece::convert_char_to_piece('x');
    }
}
