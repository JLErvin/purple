use crate::board_state::board::BoardState;
use crate::common::chess_move::Move;
use crate::common::piece::PieceType;
use crate::common::square::SquareIndex::{A1, D4};
use crate::magic::magic::MagicTable;
use crate::magic::random::MagicRandomizer;
use crate::magic::util::MagicPiece;
use crate::move_gen::perft::perft;
use crate::search::alpha_beta::AlphaBeta;
use crate::search::minimax::MinimaxSearcher;
use crate::search::search::Searcher;
use crate::uci::interface::uci_loop;
use board_state::fen::*;
use clap::*;
use common::lookup::Lookup;
use itertools::Itertools;
use rand::rngs::ThreadRng;
use std::env;
use std::time::Instant;
use crate::search::eval::INF;

mod board_state;
mod common;
mod magic;
mod move_gen;
mod search;
mod table;
mod uci;

fn main() {
    let matches = App::new("purple")
        .author("Joshua L Ervin")
        .about("A UCI chess engine")
        .arg(
            Arg::with_name("perft")
                .short("p")
                .long("perft")
                .help("run a performance test on the move generator")
                .number_of_values(2)
                .value_names(&*vec!["depth", "fen"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("alpha-perft")
                .short("a")
                .long("alpha-perft")
                .help("run a performance test on the alpha-beta searcher")
                .number_of_values(2)
                .value_names(&*vec!["depth", "fen"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("mini-perft")
                .short("m")
                .long("mini-perft")
                .help("run a performance test on the alpha-beta searcher")
                .number_of_values(2)
                .value_names(&*vec!["depth", "fen"])
                .takes_value(true),
        )
        .get_matches();

    if matches.is_present("perft") {
        execute_perft(matches.values_of("perft").unwrap().collect_vec());
        return;
    };

    if matches.is_present("mini-perft") {
        execute_mini_perft(matches.values_of("mini-perft").unwrap().collect_vec());
        return;
    };

    if matches.is_present("alpha-perft") {
        execute_alpha_perft(matches.values_of("alpha-perft").unwrap().collect_vec());
        return;
    };

    uci_loop();

    /*
    let mut searcher: AlphaBeta = Searcher::new();
    let mut pos = parse_fen(&"rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq - 0 1".to_string()).unwrap();
    let mv = searcher.best_move_depth(&mut pos, 7);
    let mut pos = parse_fen(&"rnbqkbnr/1ppppppp/p7/8/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 1 2".to_string()).unwrap();
    let mv = searcher.best_move_depth(&mut pos, 7);
    let mut pos = parse_fen(&"rnbqkbnr/1ppppppp/8/p7/3P4/1PN5/P1P1PPPP/R1BQKBNR b KQkq - 0 3".to_string()).unwrap();
    let mv = searcher.best_move_depth(&mut pos, 8);

    let mut pos = parse_fen(&"rnbqkbnr/2pppppp/1p6/p7/3P4/1PN5/PBP1PPPP/R2QKBNR b KQkq - 1 4".to_string()).unwrap();
    for mv in searcher.gen.all_moves(&mut pos) {
        let mut new_pos = pos.clone_with_move(mv);
        let s = searcher.table_fetch_debug(&mut new_pos, -INF, INF, 0);
        println!("{:?}", s);
    }

    // Bad stuff starts here
    let mv = searcher.best_move_depth(&mut pos, 7);
    println!("{}", mv.eval);
    println!("{}", mv.mv.to);
    let mut pos = parse_fen(&"rnbqkbnr/2pppppp/1p6/8/p2P4/1PN5/PBP1PPPP/R2QKBNR w KQkq - 0 5".to_string()).unwrap();
    searcher.use_table(false);
    let mv = searcher.best_move_depth(&mut pos, 6);
    println!("{}", mv.eval);
    println!("{}", mv.mv.to);

     */
}

fn execute_perft(args: Vec<&str>) {
    let depth = args.get(0).unwrap().parse::<usize>().unwrap();
    let fen = args.get(1).unwrap();

    let pos = parse_fen(fen).unwrap();

    let nodes = perft(&pos, depth);
    println!("Nodes: {}", nodes);
}

fn execute_mini_perft(args: Vec<&str>) {
    let depth = args.get(0).unwrap().parse::<usize>().unwrap();
    let fen = args.get(1).unwrap();

    let mut pos = parse_fen(fen).unwrap();

    let mut searcher = MinimaxSearcher::new();
    let mv = searcher.best_move_depth(&mut pos, depth);

    let stats = searcher.stats();
    println!("Explored {} nodes", stats.nodes);
    println!("Best Move {}", mv.mv.to_algebraic());
    println!("Move Evaluation {}", mv.eval);
}

fn execute_alpha_perft(args: Vec<&str>) {
    let depth = args.get(0).unwrap().parse::<usize>().unwrap();
    let fen = args.get(1).unwrap();

    let mut pos = parse_fen(fen).unwrap();

    let mut searcher = AlphaBeta::new();
    let mv = searcher.best_move_depth(&mut pos, depth);

    let stats = searcher.stats();
    println!("Explored {} nodes", stats.nodes);
    println!("Best Move {}", mv.mv.to_algebraic());
    println!("Move Evaluation {}", mv.eval);
}
