use crate::board_state::board::BoardState;
use crate::components::bitboard::{Bitboard, ClearBit, GetBit, Shift, RANK3, RANK7};
use crate::components::chess_move::{Move, MoveType, NORTH};
use crate::components::piece::{Color, PieceType};

use super::pawns::gen_pseudo_legal_pawn_moves;
use crate::components::chess_move::MoveType::{Capture, CastleKing, CastleQueen, EnPassantCapture};
use crate::components::piece::PieceType::{King, Knight, Queen};
use crate::components::square::rank_file_to_index;
use crate::magic::random::{GenerationScheme, MagicRandomizer};
use crate::move_gen::legal::{attacks_to, calculate_blockers, is_legal, king_square};
use crate::move_gen::lookup::Lookup;
use crate::move_gen::moves::{gen_pseudo_legal_castles, gen_pseudo_legal_moves};
use itertools::Itertools;
use rayon::prelude::IntoParallelIterator;
use std::time::Instant;

const MAX_MOVES: usize = 256;

pub fn gen_all_moves(pos: &mut BoardState, depth: usize) -> usize {
    let tic = Instant::now();
    let random = MagicRandomizer::new(GenerationScheme::PreComputed);
    let lookup = Lookup::new(random);
    let toc = tic.elapsed().as_secs_f64();
    println!("Took {} seconds", toc);
    let tic = Instant::now();
    let sum = gen(pos, depth, &lookup);
    let toc = tic.elapsed().as_secs_f64();
    println!("Took {} seconds", toc);
    println!("number of nodes at depth {}", sum);
    sum
}

pub fn gen(pos: &mut BoardState, depth: usize, lookup: &Lookup) -> usize {
    let mut list: Vec<Move> = Vec::with_capacity(MAX_MOVES);

    let king_square = king_square(pos);
    let checkers = attacks_to(pos, king_square, lookup);

    gen_pseudo_legal_pawn_moves(pos, &mut list);
    gen_pseudo_legal_castles(pos, &mut list);

    gen_pseudo_legal_moves(pos, &mut list, &lookup, PieceType::Knight);
    gen_pseudo_legal_moves(pos, &mut list, &lookup, PieceType::Rook);
    gen_pseudo_legal_moves(pos, &mut list, &lookup, PieceType::Bishop);
    gen_pseudo_legal_moves(pos, &mut list, &lookup, PieceType::Queen);

    gen_pseudo_legal_moves(pos, &mut list, &lookup, PieceType::King);

    let blockers = calculate_blockers(pos, lookup, king_square);
    list.retain(|mv| is_legal(pos, mv, &lookup, blockers, checkers, king_square));

    if depth == 1 {
        return list.len();
    } else {
        let mut sum = 0;
        for mv in list.into_iter() {
            let mut new_pos = pos.clone();
            new_pos.make_move(mv);
            sum += gen(&mut new_pos, depth - 1, lookup);
        }
        sum
    }
}

pub fn debug_print(pos: &BoardState) {
    for i in 0..8 {
        for j in 0..8 {
            let file = j;
            let rank = 7 - i;
            let square = rank_file_to_index(rank, file);
            let piece = pos.type_on(square);
            let mut c = '.';
            if piece == None {
                c = '.';
            } else {
                c = match piece.unwrap() {
                    PieceType::Pawn => 'p',
                    PieceType::Rook => 'r',
                    PieceType::Knight => 'n',
                    PieceType::Bishop => 'b',
                    PieceType::King => 'k',
                    PieceType::Queen => 'q',
                };
            }
            print!("{}", c);
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board_state::board::BoardState;
    use crate::board_state::fen::parse_fen;
    use crate::move_gen::generator::gen_all_moves;

    #[test]
    #[ignore]
    fn perft_starting_position() {
        let mut pos = BoardState::default();
        let depth_1 = gen_all_moves(&mut pos, 1);
        let depth_2 = gen_all_moves(&mut pos, 2);
        let depth_3 = gen_all_moves(&mut pos, 3);
        let depth_4 = gen_all_moves(&mut pos, 4);
        let depth_5 = gen_all_moves(&mut pos, 5);

        assert_eq!(depth_1, 20);
        assert_eq!(depth_2, 400);
        assert_eq!(depth_3, 8902);
        assert_eq!(depth_4, 197281);
    }
    #[test]
    #[ignore]
    fn perft_kiwipete() {
        let mut pos = parse_fen(
            &"r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string(),
        )
        .unwrap();

        let depth_1 = gen_all_moves(&mut pos, 1);
        let depth_2 = gen_all_moves(&mut pos, 2);
        let depth_3 = gen_all_moves(&mut pos, 3);
        let depth_4 = gen_all_moves(&mut pos, 4);

        assert_eq!(depth_1, 48);
        assert_eq!(depth_2, 2039);
        assert_eq!(depth_3, 97862);
        assert_eq!(depth_4, 4085603);
    }

    #[test]
    #[ignore]
    fn perft_fen_3() {
        let mut pos = parse_fen(&"8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1".to_string()).unwrap();

        let depth_1 = gen_all_moves(&mut pos, 1);
        let depth_2 = gen_all_moves(&mut pos, 2);
        let depth_3 = gen_all_moves(&mut pos, 3);
        let depth_4 = gen_all_moves(&mut pos, 4);
        let depth_5 = gen_all_moves(&mut pos, 5);

        assert_eq!(depth_1, 14);
        assert_eq!(depth_2, 191);
        assert_eq!(depth_3, 2812);
        assert_eq!(depth_4, 43238);
        assert_eq!(depth_5, 674624);
    }

    #[test]
    #[ignore]
    fn perft_fen_4() {
        let mut pos = parse_fen(
            &"r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string(),
        )
        .unwrap();

        let depth_1 = gen_all_moves(&mut pos, 1);
        let depth_2 = gen_all_moves(&mut pos, 2);
        let depth_3 = gen_all_moves(&mut pos, 3);
        let depth_4 = gen_all_moves(&mut pos, 4);

        assert_eq!(depth_1, 6);
        assert_eq!(depth_2, 264);
        assert_eq!(depth_3, 9467);
        assert_eq!(depth_4, 422333);
    }

    #[test]
    #[ignore]
    fn perft_fen_5() {
        let mut pos =
            parse_fen(&"rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8".to_string())
                .unwrap();

        let depth_1 = gen_all_moves(&mut pos, 1);
        let depth_2 = gen_all_moves(&mut pos, 2);
        let depth_3 = gen_all_moves(&mut pos, 3);
        let depth_4 = gen_all_moves(&mut pos, 4);

        assert_eq!(depth_1, 44);
        assert_eq!(depth_2, 1486);
        assert_eq!(depth_3, 62379);
        assert_eq!(depth_4, 2103487);
    }

    #[test]
    #[ignore]
    fn perft_fen_6() {
        let mut pos = parse_fen(
            &"r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10".to_string(),
        )
        .unwrap();

        let depth_1 = gen_all_moves(&mut pos, 1);
        let depth_2 = gen_all_moves(&mut pos, 2);
        let depth_3 = gen_all_moves(&mut pos, 3);

        assert_eq!(depth_1, 46);
        assert_eq!(depth_2, 2079);
        assert_eq!(depth_3, 89890);
    }

    #[test]
    #[ignore]
    fn perft_fen_random() {
        let mut pos =
            parse_fen(&"r6r/1bp2pP1/R2qkn2/1P6/1pPQ4/1B3N2/1B1P2p1/4K2R b KQ c3 0 1".to_string())
                .unwrap();

        let depth_1 = gen_all_moves(&mut pos, 1);
        let depth_2 = gen_all_moves(&mut pos, 2);
        let depth_3 = gen_all_moves(&mut pos, 3);

        assert_eq!(depth_1, 51);
        assert_eq!(depth_2, 2778);
        assert_eq!(depth_3, 111425);
    }
}
