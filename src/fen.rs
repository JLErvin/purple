use crate::position::*;

pub fn parse_fen(fen: &str) -> Result<Position, String> {
    let mut s: Vec<&str> = fen.split_ascii_whitespace().collect();
    if s.len() != 6 {
        return Err("FEN does not have exactly 6 fields, is invalid".to_string());
    }

    let p = parse_ranks(s.get(0).unwrap());

    let mut p = match p {
        Ok(pos) => pos,
        Err(_error) => panic!("AHHHHH"),
    };

    p.white_to_move = active_color_is_white(s.get(1).unwrap());

    Ok(p)
}

fn parse_ranks(fen: &str) -> Result<Position, String> {
    let mut p = Position::empty();
    let s: Vec<&str> = fen.split("/").collect();
    if s.len() != 8 {
        return Err("FEN position does not have exactly 8 ranks, is invalid".to_string());
    }

    for (rank, contents) in s.into_iter().enumerate() {
        let real_rank = 8 - rank;
        let mut file = 1;
        for c in contents.chars() {
            println!("RANK {} FILE {} CHAR {}", real_rank, file, c);
            match c {
                'p' | 'r' | 'n' | 'b' | 'k' | 'q' => p.add_piece(c, real_rank as u8, file),
                'P' | 'R' | 'N' | 'B' | 'K' | 'Q' => p.add_piece(c, real_rank as u8, file),
                '1'..='8' => file += char::to_digit(c, 10).unwrap() as u8,
                _ => ()
            }
            if char::is_alphabetic(c) {
                file += 1;
                println!("adding\n one");
            }

        }
    }

    p.debug_print();

    Ok(p)
}

fn active_color_is_white(fen: &str) -> bool {
    match fen.chars().next() {
        Some(c) => c == 'w',
        None => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_default_board() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let position = parse_fen(&fen.to_string());
        assert_eq!(position.unwrap().all(), 18446462598732906495);
    }

    #[test]
    fn parses_random_board_1() {
        let fen = "5K1b/8/2P1q1P1/2p5/p2N2p1/7P/2QRPP2/k6B w - - 0 1";
        let position = parse_fen(&fen.to_string());
        assert_eq!(position.unwrap().all(), 360334289566514305);
    }

    #[test]
    fn parses_random_board_2() {
        let fen = "1k1K4/1p4PB/2p3pP/6P1/1P2R3/8/rp3b2/1b4Q1 w - - 0 1";
        let position = parse_fen(&fen.to_string()).unwrap();
        assert_eq!(position.our_pawns(), 564059128725504);
        assert_eq!(position.our_rooks(), 134217728);
        assert_eq!(position.our_knights(), 0);
        assert_eq!(position.our_bishops(), 281474976710656);
        assert_eq!(position.our_king(), 1152921504606846976);
        assert_eq!(position.our_queen(), 2);
        assert_eq!(position.their_pawns(), 18051781904842752);
        assert_eq!(position.their_rooks(), 32768);
        assert_eq!(position.their_knights(), 0);
        assert_eq!(position.their_bishops(), 1088);
        assert_eq!(position.their_king(), 4611686018427387904);
        assert_eq!(position.their_queen(), 0);
        assert_eq!(position.all(), 5783504839178765378);
    }
}
