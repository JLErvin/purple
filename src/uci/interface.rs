use crate::board_state::board::BoardState;
use crate::board_state::fen::parse_fen;
use crate::components::chess_move::Move;
use crate::move_gen::generator::all_moves;
use itertools::Itertools;
use rand::Rng;
use std::io::{self, stdin, BufRead, Read};
use std::process;

pub fn uci_loop() {
    let mut pos = BoardState::default();
    loop {
        let mut buffer = String::new();
        stdin().lock().read_line(&mut buffer).unwrap();
        let key = buffer.split_ascii_whitespace().collect_vec();
        match &key.get(0).unwrap().to_string()[..] {
            "quit" => break,
            "uci" => init_uci(),
            "position" => pos = update_position(&key[1..].join(" ")),
            "go" => go(&pos),
            "isready" => println!("readyok"),
            "ucinewgame" => pos = update_position(&"startpos".to_string()),
            _ => println!("Command not understood"),
        }
    }
}

fn go(pos: &BoardState) {
    let mut rng = rand::thread_rng();
    let moves = all_moves(pos);
    let index = rng.gen_range(0..moves.len());
    let mv = moves.get(index).unwrap();
    println!("bestmove {}", mv.to_algebraic());
}

fn update_position(fen: &String) -> BoardState {
    let v = fen.split_ascii_whitespace().collect_vec();
    let keyword = v.get(0).unwrap();
    let mut pos = match &keyword[..] {
        "startpos" => BoardState::default(),
        "fen" => parse_fen(&fen[1..]).unwrap(),
        _ => panic!("Unknown parameter to position!"),
    };

    let keyword = v.get(1).unwrap_or(&"none");

    println!("  keyword is {}", keyword);

    match &keyword[..] {
        "moves" => apply_moves(&mut pos, &v[2..]),
        "none" | _ => {}
    };

    pos
}

fn apply_moves(pos: &mut BoardState, moves: &[&str]) {
    for mv_str in moves.iter() {
        let move_list = all_moves(pos);
        println!("  attempting to apply move {}", mv_str);
        let mv = move_list.iter().find(|x| x.to_algebraic() == *mv_str);
        match mv {
            None => println!("  Move not found"),
            Some(_) => println!("  Move found!"),
        };
        pos.make_move(*mv.unwrap());
    }
}

fn init_uci() {
    println!("id name Purple");
    println!("id author Joshua L Ervin");
    println!("uciok")
}
