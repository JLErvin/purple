use crate::common::piece::Color;

#[derive(Copy, Clone)]
pub enum CastleSide {
    KingSide,
    QueenSide,
}

#[derive(Copy, Clone)]
pub struct Castle {
    pub white_king: bool,
    pub white_queen: bool,
    pub black_king: bool,
    pub black_queen: bool,
}

impl Castle {
    pub fn remove_rights(&mut self, color: Color) {
        match color {
            Color::White => {
                self.white_king = false;
                self.white_queen = false;
            }
            Color::Black => {
                self.black_king = false;
                self.black_queen = false;
            }
        }
    }

    pub fn default() -> Castle {
        Castle {
            white_king: true,
            white_queen: true,
            black_king: true,
            black_queen: true,
        }
    }
}
