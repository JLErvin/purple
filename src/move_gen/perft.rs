use crate::board_state::board::BoardState;
use crate::move_gen::generator::MoveGenerator;

pub fn perft(pos: &BoardState, depth: usize) -> usize {
    let gen = MoveGenerator::new();
    perft_inner(pos, &gen, depth)
}

fn perft_inner(pos: &BoardState, gen: &MoveGenerator, depth: usize) -> usize {
    let moves = gen.all_moves(pos);
    if depth == 1 {
        moves.len()
    } else {
        let mut sum = 0;
        for mv in moves.into_iter() {
            let new_pos = pos.clone_with_move(mv);
            sum += perft_inner(&new_pos, gen, depth - 1);
        }
        sum
    }
}
