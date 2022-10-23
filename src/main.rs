#![warn(clippy::pedantic)]

use clap::*;
use itertools::Itertools;
use purple::{self, Game};

use crate::uci::uci_loop;

mod bitboard;
mod board;
mod chess_move;
mod fen;
mod magic;
mod move_gen;
mod piece;
mod search;
mod square;
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
                .help("run a performance test on the minimax searcher")
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
}

fn execute_perft(args: Vec<&str>) {
    let depth = args.get(0).unwrap().parse::<usize>().unwrap();
    let fen = args.get(1).unwrap();

    let game = Game::from_fen(fen).unwrap();
    let nodes = game.perft(depth);

    println!("Nodes: {}", nodes);
}

fn execute_mini_perft(args: Vec<&str>) {
    let depth = args.get(0).unwrap().parse::<usize>().unwrap();
    let fen = args.get(1).unwrap();

    let mut game = Game::from_fen(fen).unwrap();
    let mv = game.best_move_depth(depth);

    let stats = game.stats();
    println!("Explored {} nodes", stats.nodes);
    println!("Best Move {}", mv.mv.to_algebraic());
    println!("Move Evaluation {}", mv.eval);
}

fn execute_alpha_perft(args: Vec<&str>) {
    let depth = args.get(0).unwrap().parse::<usize>().unwrap();
    let fen = args.get(1).unwrap();

    let mut game = Game::from_fen(fen).unwrap();
    let mv = game.best_move_depth(depth);

    let stats = game.stats();
    println!("Explored {} nodes", stats.nodes);
    println!("Best Move {}", mv.mv.to_algebraic());
    println!("Move Evaluation {}", mv.eval);
}
