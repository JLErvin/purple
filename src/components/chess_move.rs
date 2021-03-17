use crate::components::piece::Piece;

pub const UP: i8 = 8;
pub const UP_LEFT: i8 = 9;
pub const UP_RIGHT: i8 = 7;

pub struct Move {
    pub to: u8,
    pub from: u8,
}
