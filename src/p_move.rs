use crate::gamestate::piece::*;

pub const UP: i8 = 8;
pub const UP_LEFT: i8 = 9;
pub const UP_RIGHT: i8 = 7;

pub struct Move {
    pub to: i8,
    pub from: i8,
    pub piece: Piece,
}
