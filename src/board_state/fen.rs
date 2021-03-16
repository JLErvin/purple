use crate::board_state::board::BoardState;
use crate::board_state::castle::*;
use crate::board_state::position::*;
use crate::board_state::*;
use crate::components::piece::Color;
use crate::components::square::*;

pub fn parse_fen(fen: &str) -> Result<BoardState, String> {
    let mut s = fen.split_whitespace();

    let position = parse_ranks(s.next().unwrap());
    let active_color = parse_active_color(s.next().unwrap());
    let castling_rights = parse_castling_rights(s.next().unwrap());
    let en_passant = parse_en_passant(s.next().unwrap());
    let half_move = parse_move(s.next().unwrap());
    let full_move = parse_move(s.next().unwrap());

    let game_state = BoardState {
        position: position.unwrap(),
        active_player: active_color,
        castling_rights,
        en_passant,
        half_move,
        full_move,
    };

    Ok(game_state)
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
                _ => (),
            }
            if char::is_alphabetic(c) {
                file += 1;
            }
        }
    }

    Ok(p)
}

fn parse_active_color(fen: &str) -> Color {
    Color::White
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

fn parse_en_passant(fen: &str) -> Square {
    8
}

fn parse_move(fen: &str) -> u8 {
    8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::piece::PieceType;

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
        let a = 5;
        assert_eq!(position.unwrap().bb_all(), 11529307423458212993);
    }

    #[test]
    fn parses_random_board_2() {
        let fen = "1k1K4/1p4PB/2p3pP/6P1/1P2R3/8/rp3b2/1b4Q1 w - - 0 1";
        let position = parse_fen(&fen.to_string()).unwrap();
        let a = 5;
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
    }
}
