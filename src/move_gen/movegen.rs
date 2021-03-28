use crate::board_state::board::BoardState;
use crate::components::bitboard::{Bitboard, ClearBit, GetBit, Shift, RANK3, RANK7};
use crate::components::chess_move::{Move, NORTH};
use crate::components::piece::PieceType;

const MAX_MOVES: usize = 256;
