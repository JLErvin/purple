use crate::board_state::board::BoardState;
use crate::board_state::fen::parse_fen;
use crate::move_gen::generator::all_moves;
use itertools::Itertools;
use rand::Rng;
use std::io::{self, stdin, BufRead, Read};

pub fn uci_loop() {
    let mut pos = BoardState::default();
    loop {
        let mut buffer = String::new();
        stdin().lock().read_line(&mut buffer).unwrap();
        println!("{}", buffer);
        let key = buffer.split_ascii_whitespace().collect_vec();
        match &key.get(0).unwrap().to_string()[..] {
            "quit" => break,
            "uci" => init_uci(),
            "position" => pos = update_position(&key[1..].join(" ")),
            "go" => go(&pos),
            _ => println!("Command not understood"),
        }
    }
}

fn go(pos: &BoardState) {
    let mut rng = rand::thread_rng();
    let moves = all_moves(pos);
    let index = rng.gen_range(0..moves.len());
    let mv = moves.get(index).unwrap();
    println!("{}", mv.to_algebraic());
}

fn update_position(fen: &String) -> BoardState {
    match &fen[..] {
        "startpos" => BoardState::default(),
        _ => parse_fen(&fen[..]).unwrap(),
    }
}

fn init_uci() {
    println!("id name Purple");
    println!("id author Joshua L Ervin");
    println!("uciok")
}
