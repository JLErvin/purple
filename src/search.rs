pub mod alpha_beta;
pub mod alpha_beta_table;
pub mod eval;
pub mod minimax;
pub mod minimax_table;
pub mod search;
pub mod par_minimax;
pub mod par_minimax_table;
pub mod alpha_beta_neg;

/*
    pub fn q_search(&mut self, board: &Board, depth: u8, mut alpha: i32, beta: i32) -> i32 {
        self.node_count += 1;
        if board.player_in_check(board.prev_move()) { return INFINITY }
        let stand_pat = board.evaluate();
        if depth == 0 || stand_pat >= beta { return stand_pat }
        if stand_pat > alpha { alpha = stand_pat }

        for (_, mv) in board.qsort(&board.get_moves()) {
            let mut new_board = *board;
            new_board.make_move(mv);
            let score = -self.q_search(&new_board, depth - 1, -beta, -alpha);

            if score > alpha {
                if score >= beta { return score }
                alpha = score;
            }
        }
        alpha
    }

    fn quiescence_search(board: &mut Board, mut alpha: i16, beta: i16, max_depth: u16) -> ScoringMove {
    if board.depth() == max_depth {
        return eval_board(board);
    }

    let moves = if board.in_check() {
        board.generate_moves()
    } else {
        board.generate_moves_of_type(GenTypes::Captures)
    };

    if moves.is_empty() {
        if board.in_check() {
            return ScoringMove::blank(-MATE_V + (board.depth() as i16));
        }
        return eval_board(board);
    }
    let mut best_move: BitMove = BitMove::null();
    for mov in moves {
        board.apply_move(mov);

        let return_move = { quiescence_search(board, -beta, -alpha, max_depth) }.negate();

        board.undo_move();

        if return_move.score > alpha {
            alpha = return_move.score;
            best_move = mov;
        }

        if alpha >= beta {
            return ScoringMove {
                bit_move: mov,
                score: alpha,
            };
        }
    }

    ScoringMove {
        bit_move: best_move,
        score: alpha,
    }
}

*/