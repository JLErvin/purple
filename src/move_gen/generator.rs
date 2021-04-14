use crate::board_state::board::BoardState;
use crate::components::bitboard::{Bitboard, ClearBit, GetBit, Shift, RANK3, RANK7};
use crate::components::chess_move::{Move, MoveType, NORTH};
use crate::components::piece::{Color, PieceType};

use super::pawns::gen_pseudo_legal_pawn_moves;
use crate::components::chess_move::MoveType::{Capture, CastleKing, CastleQueen, EnPassantCapture};
use crate::components::piece::PieceType::{King, Knight, Queen};
use crate::components::square::rank_file_to_index;
use crate::magic::random::{GenerationScheme, MagicRandomizer};
use crate::move_gen::legal::is_legal;
use crate::move_gen::lookup::Lookup;
use crate::move_gen::moves::{gen_pseudo_legal_castles, gen_pseudo_legal_moves};
use itertools::Itertools;
use rayon::prelude::IntoParallelIterator;
use std::time::Instant;

const MAX_MOVES: usize = 256;

pub fn gen_all_pseudo_legal_moves(pos: &mut BoardState) -> usize {
    let random = MagicRandomizer::new(GenerationScheme::PreComputed);
    let lookup = Lookup::new(random);
    let tic = Instant::now();
    let sum = gen(pos, 6, &lookup);
    let toc = tic.elapsed().as_secs_f64();
    println!("Took {} seconds", toc);
    println!("number of nodes at depth {}", sum);
    sum
}

pub fn gen(pos: &mut BoardState, depth: usize, lookup: &Lookup) -> usize {
    let mut list: Vec<Move> = Vec::with_capacity(MAX_MOVES);

    gen_pseudo_legal_pawn_moves(pos, &mut list);
    gen_pseudo_legal_castles(pos, &mut list);

    gen_pseudo_legal_moves(pos, &mut list, &lookup, PieceType::King);
    gen_pseudo_legal_moves(pos, &mut list, &lookup, PieceType::Knight);
    gen_pseudo_legal_moves(pos, &mut list, &lookup, PieceType::Rook);
    gen_pseudo_legal_moves(pos, &mut list, &lookup, PieceType::Bishop);
    gen_pseudo_legal_moves(pos, &mut list, &lookup, PieceType::Queen);

    let v = list
        .into_iter()
        .filter(|x| is_legal(pos, &x, &lookup))
        .collect_vec();

    if depth == 0 {
        for mv in v.iter() {
            //println!("MOVE");
            if mv.kind == EnPassantCapture {
                //println!("EN PASSANT");
            } else if mv.kind == Capture {
                //println!("CAPTURE");
            } else if mv.kind == CastleKing || mv.kind == CastleQueen {
                //println!("CASTLE");
            }
            let pt = pos.type_on(mv.from).unwrap();
            let p = match pt {
                PieceType::Pawn => "P",
                PieceType::Rook => "R",
                PieceType::King => "K",
                PieceType::Bishop => "B",
                PieceType::Queen => "Q",
                PieceType::Knight => "N",
                _ => "",
            };

            //if pt == Queen && mv.to == 12 && mv.from == 21 {
            //    println!("{}", pos.bb_all());
            //}
            if mv.to == 62 && mv.from == 60 && pos.bb_all() == 10474584692977106833 {
                //let mut new_pos = pos.clone();
                //new_pos.make_move(*mv);
                //println!("Preview Pos: {}", pos.bb_all());
                //println!("New Pos    : {}", new_pos.bb_all());
                //println!("Capture");
                //println!("{}", pos.bb_all());
                //debug_print(pos);
            }
            //println!("{} from {} to {}", p, mv.from, mv.to);
        }

        return v.len();
    } else {
        let mut sum = 0;
        for mv in v.into_iter() {
            /*            let pt = pos.type_on(mv.from).unwrap();
                        let p = match pt {
                            PieceType::Pawn => "P",
                            PieceType::Rook => "R",
                            PieceType::King => "K",
                            _ => "",
                        };
                        let mut new_pos = pos.clone();
                        println!("{} from {} to {}", p, mv.from, mv.to);
            */
            let mut new_pos = pos.clone();
            new_pos.make_move(mv);
            sum += gen(&mut new_pos, depth - 1, lookup);
        }
        sum
    }
}

fn debug_print(pos: &BoardState) {
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
    use crate::board_state::fen::parse_fen;
    use crate::move_gen::generator::gen_all_pseudo_legal_moves;

    #[test]
    fn generates_kiwi_pete() {
        let mut pos = parse_fen(
            &"r3k2r/p1ppq1b1/bn2pn2/3P2N1/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 2".to_string(),
        )
        .unwrap();
        let sum = gen_all_pseudo_legal_moves(&mut pos);
        //assert_eq!(sum, 2039);
    }
}
