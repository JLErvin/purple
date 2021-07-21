use crate::board_state::board::BoardState;
use crate::board_state::position::Position;
use crate::common::bitboard::{Bitboard, PieceItr};
use crate::common::chess_move::MoveType::{Capture, Quiet};
use crate::common::chess_move::{Move, MoveType};
use crate::common::lookup::Lookup;
use crate::common::piece::{Color, PieceType};
use crate::common::square::SquareIndex::{C1, C8, E1, E8, G1, G8};
use crate::move_gen::util::extract_moves;
use std::io::empty;

pub fn gen_pseudo_legal_moves(
    pos: &BoardState,
    list: &mut Vec<Move>,
    lookup: &Lookup,
    piece: PieceType,
) {
    let us = pos.active_player();
    let pieces = pos.bb(us, piece);
    let valid_pieces = pos.bb_for_color(!us);
    let empty_squares = !pos.bb_all();

    for (square, _) in pieces.iter() {
        let destinations = match piece {
            PieceType::King | PieceType::Knight => lookup.moves(square, piece),
            _ => lookup.sliding_moves(square, pos.bb_all(), piece),
        };
        let captures = destinations & valid_pieces;
        let quiets = destinations & empty_squares;

        extract_moves(square, captures, list, Capture);
        extract_moves(square, quiets, list, Quiet);
    }
}

pub fn gen_pseudo_legal_castles(pos: &BoardState, list: &mut Vec<Move>) {
    let us = pos.active_player();

    let (king_mask, queen_mask) = match us {
        Color::White => (96, 14),
        Color::Black => (6917529027641081856, 1008806316530991104),
    };

    let occupied = pos.bb_all();

    let (king_rights, queen_rights) = match us {
        Color::White => (
            pos.castling_rights().white_king,
            pos.castling_rights().white_queen,
        ),
        Color::Black => (
            pos.castling_rights().black_king,
            pos.castling_rights().black_queen,
        ),
    };

    if (occupied & king_mask == 0) && king_rights {
        let (to, from) = match us {
            Color::White => (G1 as u8, E1 as u8),
            Color::Black => (G8 as u8, E8 as u8),
        };
        let m = Move {
            to,
            from,
            kind: MoveType::CastleKing,
        };
        list.push(m);
    }

    if (occupied & queen_mask == 0) && queen_rights {
        let (to, from) = match us {
            Color::White => (C1 as u8, E1 as u8),
            Color::Black => (C8 as u8, E8 as u8),
        };
        let m = Move {
            to,
            from,
            kind: MoveType::CastleQueen,
        };
        list.push(m);
    }
}

#[cfg(test)]
mod test {
    use crate::board_state::fen::parse_fen;
    use crate::common::chess_move::Move;
    use crate::move_gen::moves::gen_pseudo_legal_castles;

    #[test]
    fn castles_no_obstruction() {
        let pos = parse_fen(&"8/8/8/8/8/8/8/R3K2R w KQ - 0 1".to_string()).unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_castles(&pos, &mut list);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn no_castles_with_obstruction() {
        let pos = parse_fen(&"8/8/8/8/8/8/8/R3KB1R w KQ - 0 1".to_string()).unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_castles(&pos, &mut list);
        assert_eq!(list.len(), 1);

        let pos = parse_fen(&"8/8/8/8/8/8/8/R1B1K2R w KQ - 0 1".to_string()).unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_castles(&pos, &mut list);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn no_castles_without_rights() {
        let pos = parse_fen(&"8/8/8/8/8/8/8/R3K2R w K - 0 1".to_string()).unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_castles(&pos, &mut list);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn black_queenside_castle() {
        let pos = parse_fen(
            &"r3k2r/p1ppq1b1/bn2pn2/3P2N1/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 2".to_string(),
        )
        .unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_castles(&pos, &mut list);
        let m1 = list.get(0).unwrap();
        let m2 = list.get(1).unwrap();
        assert_eq!(list.len(), 2);
    }
}
