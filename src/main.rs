use crate::board_state::board::BoardState;
use crate::components::chess_move::Move;
use crate::components::piece::PieceType;
use crate::components::square::SquareIndex::{A1, D4};
use crate::magic::magic::MagicTable;
use crate::magic::random::MagicRandomizer;
use crate::magic::util::MagicPiece;
use crate::move_gen::lookup::Lookup;
use crate::uci::interface::uci_loop;
use board_state::fen::*;
use clap::*;
use itertools::Itertools;
use move_gen::generator::perft;
use rand::rngs::ThreadRng;
use std::env;
use std::time::Instant;

mod board_state;
mod components;
mod magic;
mod move_gen;
mod uci;

fn main() {
    println!("Hello, world!");

    let matches = App::new("purple")
        .author("Joshua L Ervin")
        .about("A UCI chess engine")
        .arg(
            Arg::with_name("perft")
                .short("p")
                .long("perft")
                .help("run a performance test")
                .number_of_values(2)
                .takes_value(true),
        )
        .get_matches();

    if matches.is_present("perft") {
        execute_perft(matches.values_of("perft").unwrap().collect_vec());
    };

    //uci_loop();

    //gen_all_moves(&mut b, depth);
    println!();
}

fn execute_perft(args: Vec<&str>) {
    let depth = args.get(0).unwrap().parse::<usize>().unwrap();
    let fen = args.get(1).unwrap();

    let pos = parse_fen(fen).unwrap();

    perft(&pos, depth);
}
