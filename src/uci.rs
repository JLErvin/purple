use std::io::{stdin, BufRead};

use itertools::Itertools;

use crate::board::BoardState;
use crate::fen::parse_fen;
use crate::move_gen::MoveGenerator;
use crate::search::alpha_beta::AlphaBeta;
use crate::search::eval::eval;
use crate::search::search::Searcher;

pub fn uci_loop() {
    let mut pos = BoardState::default();
    let mut searcher = AlphaBeta::new();
    loop {
        let mut buffer = String::new();
        stdin().lock().read_line(&mut buffer).unwrap();
        let key = buffer.split_ascii_whitespace().collect_vec();
        match &(*key.first().unwrap()).to_string()[..] {
            "quit" => break,
            "uci" => init_uci(),
            "position" => pos = update_position(&key[1..].join(" ")),
            "go" => go(&mut pos, &mut searcher, &key),
            "isready" => println!("readyok"),
            "ucinewgame" => pos = update_position(&"startpos".to_string()),
            //"movetime" => searcher.move_time(key[1].parse::<u64>().unwrap()),
            _ => println!("Command not understood"),
        }
    }
}

fn go(pos: &mut BoardState, searcher: &mut AlphaBeta, data: &[&str]) {
    let movetime = data[2].parse::<u128>().unwrap();
    //searcher.move_time((movetime / 1000) - 1);
    searcher.move_time(movetime);
    let mv = searcher.best_move_depth(pos, 15);
    println!("eval: {}", mv.eval);
    println!("static eval: {}", eval(pos));
    println!("bestmove {}", mv.mv.to_algebraic());
}

fn update_position(fen: &String) -> BoardState {
    let v = fen.split_ascii_whitespace().collect_vec();
    let keyword = v.first().unwrap();
    let mut pos = match &keyword[..] {
        "startpos" => BoardState::default(),
        "fen" => return parse_fen(&fen[4..]).unwrap(),
        _ => panic!("Unknown parameter to position!"),
    };

    let keyword = v.get(1);

    if keyword.is_some() {
        apply_moves(&mut pos, &v[2..]);
    }

    pos
}

fn apply_moves(pos: &mut BoardState, moves: &[&str]) {
    for mv_str in moves.iter() {
        let gen = MoveGenerator::new();
        let move_list = gen.all_moves(pos);
        let mv = move_list.iter().find(|x| x.to_algebraic() == *mv_str);
        pos.make_move(*mv.unwrap());
    }
}

fn init_uci() {
    println!("id name Purple_Threefold");
    println!("id author Joshua L Ervin");
    println!("uciok");
}
