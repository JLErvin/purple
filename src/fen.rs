use crate::board::{BoardState, Castle, Position};
use crate::common::square::{algebraic_to_square, Square};
use crate::piece::Color;

pub fn parse_fen(fen: &str) -> Result<BoardState, String> {
    let mut s = fen.split_whitespace();
    println!("FEN: {}", fen);

    let position = parse_ranks(s.next().unwrap());
    let active_color = parse_active_color(s.next().unwrap());
    let castling_rights = parse_castling_rights(s.next().unwrap());
    let en_passant = parse_en_passant(s.next().unwrap());
    let half_move = parse_move(s.next().unwrap());
    let full_move = parse_move(s.next().unwrap());

    let board_state = BoardState {
        position: position.unwrap(),
        active_player: active_color.unwrap(),
        castling_rights,
        en_passant,
        half_move,
        full_move,
    };

    Ok(board_state)
}

fn parse_ranks(fen: &str) -> Result<Position, String> {
    let mut p = Position::empty();
    let s: Vec<&str> = fen.split('/').collect();
    if s.len() != 8 {
        return Err("FEN position does not have exactly 8 ranks, is invalid".to_string());
    }

    for (rank, contents) in s.into_iter().enumerate() {
        let real_rank = 7 - rank;
        let mut file = 0;
        for c in contents.chars() {
            match c {
                'p' | 'r' | 'n' | 'b' | 'k' | 'q' => p.add_piece(c, real_rank as u8, file),
                'P' | 'R' | 'N' | 'B' | 'K' | 'Q' => p.add_piece(c, real_rank as u8, file),
                '1'..='8' => file += char::to_digit(c, 10).unwrap() as u8,
                _ => return Err("FEN contains illegal character in board description".to_string()),
            }
            if char::is_alphabetic(c) {
                file += 1;
            }
        }
    }

    Ok(p)
}

fn parse_active_color(fen: &str) -> Result<Color, String> {
    let c = match fen.chars().next().unwrap() {
        'w' => Ok(Color::White),
        'b' => Ok(Color::Black),
        _ => Err("Cannot parse active color".to_string()),
    };
    c
}

fn parse_castling_rights(fen: &str) -> Castle {
    let mut white_king = false;
    let mut white_queen = false;
    let mut black_king = false;
    let mut black_queen = false;
    for c in fen.chars() {
        match c {
            'K' => white_king = true,
            'Q' => white_queen = true,
            'k' => black_king = true,
            'q' => black_queen = true,
            _ => (),
        }
    }
    Castle {
        white_king,
        white_queen,
        black_king,
        black_queen,
    }
}

fn parse_en_passant(fen: &str) -> Option<Square> {
    let c = fen.chars().next().unwrap();
    match c {
        '-' => None,
        _ => Some(algebraic_to_square(&fen[0..2])),
    }
}

fn parse_move(fen: &str) -> u8 {
    fen.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::PieceType;

    #[test]
    fn parses_default_board() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let position = parse_fen(&fen.to_string());
        assert_eq!(position.unwrap().bb_all(), 18446462598732906495);
    }

    #[test]
    fn parses_random_board_1() {
        let fen = "5K1b/8/2P1q1P1/2p5/p2N2p1/7P/2QRPP2/k6B w - - 0 1";
        let position = parse_fen(&fen.to_string());
        assert_eq!(position.unwrap().bb_all(), 11529307423458212993);
    }

    #[test]
    fn parses_random_board_2() {
        let fen = "1k1K4/1p4PB/2p3pP/6P1/1P2R3/8/rp3b2/1b4Q1 w - - 0 1";
        let position = parse_fen(&fen.to_string()).unwrap();
        assert_eq!(
            position.bb(Color::White, PieceType::Pawn),
            18155410909298688
        );
        assert_eq!(position.bb(Color::White, PieceType::Rook), 268435456);
        assert_eq!(position.bb(Color::White, PieceType::Knight), 0);
        assert_eq!(
            position.bb(Color::White, PieceType::Bishop),
            36028797018963968
        );
        assert_eq!(
            position.bb(Color::White, PieceType::King),
            576460752303423488
        );
        assert_eq!(position.bb(Color::White, PieceType::Queen), 64);
        assert_eq!(position.bb(Color::Black, PieceType::Pawn), 637716744110592);
        assert_eq!(position.bb(Color::Black, PieceType::Rook), 256);
        assert_eq!(position.bb(Color::Black, PieceType::Knight), 0);
        assert_eq!(position.bb(Color::Black, PieceType::Bishop), 8194);
        assert_eq!(
            position.bb(Color::Black, PieceType::King),
            144115188075855872
        );
        assert_eq!(position.bb(Color::Black, PieceType::Queen), 0);
        assert_eq!(position.bb_all(), 775397865320096578);
        assert_eq!(position.active_player, Color::White);
    }

    #[test]
    fn parses_random_board_3() {
        let fen = "3r2r1/P6b/q2pKPk1/4P3/1p1P1R2/5n2/1B2N3/8 w - - 0 1";
        let position = parse_fen(&fen.to_string()).unwrap();
        assert_eq!(position.bb_all(), 5224590153059668480);
    }

    #[test]
    fn parses_active_black() {
        let fen = "1k1K4/1p4PB/2p3pP/6P1/1P2R3/8/rp3b2/1b4Q1 b - - 0 1";
        let position = parse_fen(&fen.to_string()).unwrap();
        assert_eq!(position.active_player, Color::Black);
    }

    #[test]
    fn parses_en_passant() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let position = parse_fen(&fen.to_string()).unwrap();
        assert_eq!(position.en_passant.unwrap(), 20);
    }

    #[test]
    fn parses_move_count() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let position = parse_fen(&fen.to_string()).unwrap();
        assert_eq!(position.half_move, 0);
        assert_eq!(position.full_move, 1);
    }

    #[test]
    #[should_panic]
    fn panics_on_incorrect_fen_ranks() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8 b KQkq e3 0 1";
        let _position = parse_fen(&fen.to_string()).unwrap();
    }

    #[test]
    #[should_panic]
    fn panics_on_incorrect_fen_color() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR Q KQkq e3 0 1";
        let _position = parse_fen(&fen.to_string()).unwrap();
    }

    #[test]
    #[should_panic]
    fn panics_on_illegal_character() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNX b KQkq e3 0 1";
        let _position = parse_fen(&fen.to_string()).unwrap();
    }
}
