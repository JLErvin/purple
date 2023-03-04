use itertools::Itertools;

use crate::bitboard::{
    AddPiece, Bitboard, New, PieceItr, Shift, FILEA, FILEB, FILEG, FILEH, RANK2, RANK3, RANK6,
    RANK7,
};
use crate::board::BoardState;
use crate::chess_move::MoveType::{Capture, EnPassantCapture, Quiet};
use crate::chess_move::{Move, MoveType, PromotionType, EAST, NORTH, SOUTH, WEST};
use crate::magic::{GenerationScheme, MagicPiece, MagicRandomizer, MagicTable};
use crate::piece::{Color, PieceType};
use crate::square::SquareIndex::{C1, C8, E1, E8, G1, G8};
use crate::square::{rank_file_to_index, Square};

const MAX_MOVES: usize = 256;

pub struct Lookup {
    rook_table: MagicTable,
    bishop_table: MagicTable,
    king_table: Vec<Bitboard>,
    knight_table: Vec<Bitboard>,
    between: [[Bitboard; 64]; 64],
    pseudo_rooks: [Bitboard; 64],
    pseudo_bishops: [Bitboard; 64],
    square: [Bitboard; 64],
}

impl Lookup {
    pub fn new(mut random: MagicRandomizer) -> Lookup {
        let rook_table = MagicTable::init(MagicPiece::Rook, &mut random);
        let bishop_table = MagicTable::init(MagicPiece::Bishop, &mut random);
        let king_table = Lookup::init_king();
        let knight_table = Lookup::init_knight();
        let between = Lookup::init_between(&rook_table, &bishop_table);
        let dumb_rooks = Lookup::init_pseudo(&rook_table);
        let dumb_bishops = Lookup::init_pseudo(&bishop_table);
        let square = Lookup::init_square();

        Lookup {
            rook_table,
            bishop_table,
            king_table,
            knight_table,
            between,
            pseudo_rooks: dumb_rooks,
            pseudo_bishops: dumb_bishops,
            square,
        }
    }

    #[inline]
    pub fn square_bb(&self, square: Square) -> Bitboard {
        self.square[square as usize]
    }

    /// Given a non-sliding piece (i.e. any piece which is not constrained in it's movement by blockers
    /// returns a bitboard representing all possible destination squares for that piece.
    pub fn moves(&self, square: Square, piece: PieceType) -> Bitboard {
        match piece {
            PieceType::Knight => *self.knight_table.get(square as usize).unwrap(),
            PieceType::King => *self.king_table.get(square as usize).unwrap(),
            PieceType::Queen => self.sliding_moves(square, 0, PieceType::Queen),
            _ => 0,
        }
    }

    /// Given a square, piece, and blockers, returns a Bitboard which represents all possible
    /// destination squares of that piece.
    pub fn sliding_moves(&self, square: Square, blockers: Bitboard, piece: PieceType) -> Bitboard {
        match piece {
            PieceType::Rook => self.rook_table.moves(square, blockers),
            PieceType::Bishop => self.bishop_table.moves(square, blockers),
            PieceType::Queen => {
                self.rook_table.moves(square, blockers) | self.bishop_table.moves(square, blockers)
            }
            _ => 0,
        }
    }

    /// Given two squares s1 and s2, returns a bitboard which represents the line which passes
    /// through both of them. If s1 and s2 are not on the same diagonal, 0 is returned.
    /// Note that such a Bitboard extends the whole length of the board (i.e. if s1=A1 and s2=B1,
    /// then the returned Bitboard is the entire first rank).
    pub fn between(&self, s1: Square, s2: Square) -> Bitboard {
        self.between[s1 as usize][s2 as usize]
    }

    fn init_king() -> Vec<Bitboard> {
        let mut v: Vec<Bitboard> = Vec::with_capacity(64);

        for i in 0..64 {
            let mut b: Bitboard = 0;
            let mut r: Bitboard = 0;
            b = b.add_at_square(i);
            for dir in MoveType::king_itr() {
                r |= b.shift(*dir);
            }
            v.push(r);
        }

        v
    }

    fn init_knight() -> Vec<Bitboard> {
        let mut v: Vec<Bitboard> = Vec::with_capacity(64);

        for i in 0..64 {
            let b = knight_destinations(i as u8);
            v.push(b);
        }
        v
    }

    fn init_pseudo(table: &MagicTable) -> [Bitboard; 64] {
        let mut t: [Bitboard; 64] = [0; 64];

        for i in 0..64 {
            t[i] = table.moves(i as u8, 0);
        }
        t
    }

    fn init_square() -> [Bitboard; 64] {
        let mut t: [Bitboard; 64] = [0; 64];

        for i in 0..64 {
            t[i] = 1 << i;
        }

        t
    }

    /// Returns a bitboard which represents the attacks of the given piece on the empty board.
    pub fn pseudo_attacks(&self, piece: PieceType, square: Square) -> Bitboard {
        match piece {
            PieceType::Rook => self.pseudo_rooks[square as usize],
            PieceType::Bishop => self.pseudo_bishops[square as usize],
            _ => 0,
        }
    }

    fn attacks(
        rook_table: &MagicTable,
        bishop_table: &MagicTable,
        square: Square,
        piece: MagicPiece,
    ) -> Bitboard {
        match piece {
            MagicPiece::Rook => rook_table.moves(square, 0),
            MagicPiece::Bishop => bishop_table.moves(square, 0),
        }
    }

    fn init_between(rook_table: &MagicTable, bishop_table: &MagicTable) -> [[Bitboard; 64]; 64] {
        let mut b: [[Bitboard; 64]; 64] = [[0; 64]; 64];

        for piece in &[MagicPiece::Rook, MagicPiece::Bishop] {
            for (i, j) in (0..64).cartesian_product(0..64) {
                let bitboard_i = Bitboard::for_square(i);
                let bitboard_j = Bitboard::for_square(j);
                let attacks_i = Lookup::attacks(rook_table, bishop_table, i, *piece);

                if attacks_i & bitboard_j != 0 {
                    match piece {
                        MagicPiece::Rook => {
                            b[i as usize][j as usize] =
                                attacks_i & rook_table.moves(j, 0) | bitboard_i | bitboard_j
                        }
                        MagicPiece::Bishop => {
                            b[i as usize][j as usize] =
                                attacks_i & bishop_table.moves(j, 0) | bitboard_i | bitboard_j
                        }
                    }
                }
            }
        }
        b
    }
}

pub struct MoveGenerator {
    pub lookup: Lookup,
}

impl MoveGenerator {
    pub fn new() -> MoveGenerator {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        MoveGenerator { lookup }
    }

    pub fn all_moves(&self, pos: &BoardState) -> Vec<Move> {
        let mut list: Vec<Move> = Vec::with_capacity(MAX_MOVES);

        gen_pseudo_legal_pawn_moves(pos, &mut list);
        gen_pseudo_legal_castles(pos, &mut list);

        gen_pseudo_legal_moves(pos, &mut list, &self.lookup, PieceType::Knight);
        gen_pseudo_legal_moves(pos, &mut list, &self.lookup, PieceType::Rook);
        gen_pseudo_legal_moves(pos, &mut list, &self.lookup, PieceType::Bishop);
        gen_pseudo_legal_moves(pos, &mut list, &self.lookup, PieceType::Queen);

        gen_pseudo_legal_moves(pos, &mut list, &self.lookup, PieceType::King);

        let king_square = king_square(pos);
        let blockers = calculate_blockers(pos, &self.lookup, king_square);
        let checkers = attacks_to(pos, king_square, &self.lookup);

        list.retain(|mv| is_legal(pos, mv, &self.lookup, blockers, checkers, king_square));

        list
    }

    #[allow(dead_code)]
    pub fn perft(&self, pos: &BoardState, depth: usize) -> usize {
        self.perft_inner(pos, depth)
    }

    fn perft_inner(&self, pos: &BoardState, depth: usize) -> usize {
        let moves = self.all_moves(pos);
        if depth == 1 {
            moves.len()
        } else {
            let mut sum = 0;
            for mv in moves {
                let new_pos = pos.clone_with_move(mv);
                sum += self.perft_inner(&new_pos, depth - 1);
            }
            sum
        }
    }
}

/// Determines whether or not the given move is legal given the provided state of the game.
/// A move is determined to be legal if it does not leave the king in check after the move is made.
pub fn is_legal(
    pos: &BoardState,
    mv: &Move,
    lookup: &Lookup,
    blockers: Bitboard,
    checkers: Bitboard,
    king_square: Square,
) -> bool {
    let from = mv.from;

    let is_castle = mv.kind == MoveType::CastleKing || mv.kind == MoveType::CastleQueen;
    if king_on_square(pos, lookup, from) && !is_castle {
        is_legal_king_move(pos, mv, lookup)
    } else {
        is_legal_non_king_move(pos, mv, lookup, blockers, checkers, king_square)
    }
}

/// Determines if the given move is legal, working under the assumption that the provided move
/// is a king move. Such a move is legal so long as the destination square of the king is not attacked
/// by the opponent's pieces.
fn is_legal_king_move(pos: &BoardState, mv: &Move, lookup: &Lookup) -> bool {
    !is_attacked(pos, mv.to, lookup)
}

/// Determines if the given move is legal, working under the assumption that the provided move
/// is not a king move. Such a move is legal under the following conditions:
/// 1. If the king is attacked by more than once piece, the move will always be illegal
/// 2. If the king is attacked by one piece, the move is legal iff we block the attack or capture
///    the attacking piece and do not expose the king in the process.
/// 3. If the given piece is pinned the move is legal only if we move along the pinned ray or capture
///    the attacking piece.
/// 4. If the king is not attacked and the piece is not pinned the move will always be legal.
fn is_legal_non_king_move(
    pos: &BoardState,
    mv: &Move,
    lookup: &Lookup,
    blockers: Bitboard,
    checkers: Bitboard,
    king_square: Square,
) -> bool {
    let num_checkers = checkers.count_ones();

    // If more than one piece has put the king in check then the only legal move is for the king to move
    // and evade checks - hence a non-king move will always be illegal.
    if num_checkers > 1 {
        return false;
    }

    let pinned = is_absolutely_pinned(mv, lookup, blockers);

    if mv.kind == MoveType::EnPassantCapture {
        return is_legal_en_passant(pos, mv, lookup, king_square);
    } else if mv.kind == MoveType::CastleKing || mv.kind == MoveType::CastleQueen {
        return is_legal_castle(pos, mv, lookup, num_checkers);
    }

    // If exactly one piece puts us in check then our move is legal iff we block the incoming attack
    // or we capture the attacking piece.
    if num_checkers == 1 {
        let piece_bb = lookup.square_bb(mv.to);
        let attacker_square = checkers.trailing_zeros() as u8;

        return if mv.to == attacker_square {
            !pinned
        } else {
            let attacking_ray = ray_between(king_square, attacker_square, lookup);
            !pinned && (attacking_ray & piece_bb != 0)
        };
    }

    // If a piece is not absolutely pinned then it is free to move anywhere since we have already
    // determined the king is not currently in check.
    if !pinned {
        return true;
    }

    is_legal_pin_move(pos, mv, lookup)
}

/// Determines whether or not the given move is legal, working under the assumption that the provided
/// move represents a castling move. En Passant requires special checking since it is the only move in
/// which the piece moves to a square but does not capture on that square.
fn is_legal_en_passant(pos: &BoardState, mv: &Move, lookup: &Lookup, king_square: Square) -> bool {
    let us = pos.active_player;
    let mut pos = *pos;

    let offset: i8 = match us {
        Color::White => 8,
        Color::Black => -8,
    };

    pos.remove_piece(PieceType::Pawn, !us, (mv.to as i8 - offset) as u8);
    let tmp_mv = Move {
        to: mv.to,
        from: mv.from,
        kind: Capture,
    };
    let blockers = calculate_blockers(&pos, lookup, king_square);
    let checkers = attacks_to(&pos, king_square, lookup);
    let is_legal = is_legal_non_king_move(&pos, &tmp_mv, lookup, blockers, checkers, king_square);
    pos.add(PieceType::Pawn, !us, (mv.to as i8 - offset) as u8);
    is_legal
}

/// Determines whether or not the given move is legal, working under the assumption that the given
/// move represents a castling move. A castle is illegal if the king is currently or would castle through a check.
fn is_legal_castle(pos: &BoardState, mv: &Move, lookup: &Lookup, num_checkers: u32) -> bool {
    if num_checkers != 0 {
        return false;
    }

    let squares: Vec<Square> = match mv.kind {
        MoveType::CastleKing => match pos.active_player {
            Color::White => vec![5, 6],
            Color::Black => vec![61, 62],
        },
        MoveType::CastleQueen => match pos.active_player {
            Color::White => vec![2, 3],
            Color::Black => vec![58, 59],
        },
        _ => vec![],
    };

    for square in squares {
        if is_attacked(pos, square, lookup) {
            return false;
        }
    }

    true
}

/// Determines whether or not the given move is legal, working under the assumption that the moved
/// piece is currently pinned. Such a move is legal iff we move along the pinning ray or we caputre
/// the attacking piece
fn is_legal_pin_move(pos: &BoardState, mv: &Move, lookup: &Lookup) -> bool {
    let ray = lookup.between(mv.to, mv.from);
    let overlap = ray & pos.bb(pos.active_player, PieceType::King);

    overlap != 0
}

/// Determines whether or not the given piece being moved is pinned. If the piece is pinned, the returned Square
/// represents the square of the pinning piece.
fn is_absolutely_pinned(mv: &Move, lookup: &Lookup, blockers: Bitboard) -> bool {
    let piece_bb = lookup.square_bb(mv.from);

    let intersect = blockers & piece_bb;

    intersect != 0
}

/// Returns a bitboard representing all pieces which are attacking the provided square.
pub fn attacks_to(pos: &BoardState, square: Square, lookup: &Lookup) -> Bitboard {
    let us = pos.active_player;
    let occupancies = pos.bb_all() & !pos.bb(us, PieceType::King);

    let pawn_attacks = pawn_attacks(square, us);
    let rook_attacks = lookup.sliding_moves(square, occupancies, PieceType::Rook);
    let bishop_attacks = lookup.sliding_moves(square, occupancies, PieceType::Bishop);
    let queen_attacks = rook_attacks | bishop_attacks;
    let knight_attacks = lookup.moves(square, PieceType::Knight);
    let king_attacks = lookup.moves(square, PieceType::King);

    let pawns = pawn_attacks & pos.bb_pieces(PieceType::Pawn);
    let rooks = rook_attacks & pos.bb_pieces(PieceType::Rook);
    let bishops = bishop_attacks & pos.bb_pieces(PieceType::Bishop);
    let queens = queen_attacks & pos.bb_pieces(PieceType::Queen);
    let knights = knight_attacks & pos.bb_pieces(PieceType::Knight);
    let king = king_attacks & pos.bb_pieces(PieceType::King);

    (pawns | rooks | bishops | queens | knights | king) & pos.bb_for_color(!us)
}

pub fn is_in_check(pos: &BoardState, lookup: &Lookup) -> bool {
    let king_square = king_square(pos);
    let checkers: Bitboard = attacks_to(pos, king_square, lookup);
    checkers.count_ones() != 0
}

/// Calculates the ray strictly inclusive between s1 and s2
fn ray_between(s1: Square, s2: Square, lookup: &Lookup) -> Bitboard {
    let full: Bitboard = !0;
    let b1 = lookup.square_bb(s1);
    let b2 = lookup.square_bb(s2);
    lookup.between(s1, s2) & ((full << s1) ^ (full << s2)) | b1 | b2
}

fn king_on_square(pos: &BoardState, lookup: &Lookup, square: Square) -> bool {
    let b = lookup.square_bb(square);
    let king = pos.bb(pos.active_player, PieceType::King);
    b & king != 0
}

/// Given the state of a game, calculates and returns a bitboard which represents all blockers
/// (i.e. pinned pieces) for the king.
pub fn calculate_blockers(pos: &BoardState, lookup: &Lookup, king_square: Square) -> Bitboard {
    let us = pos.active_player;
    let king_bb = pos.bb(us, PieceType::King);

    let attacks_rooks = lookup.pseudo_attacks(PieceType::Rook, king_square)
        & (pos.bb(!us, PieceType::Rook) | pos.bb(!us, PieceType::Queen));
    let attacks_bishops = lookup.pseudo_attacks(PieceType::Bishop, king_square)
        & (pos.bb(!us, PieceType::Bishop) | pos.bb(!us, PieceType::Queen));

    let snipers = (attacks_rooks | attacks_bishops) & !pos.bb(us, PieceType::King);
    let occupancy = pos.bb_all();

    let mut blockers = Bitboard::empty();

    for (i, _) in snipers.iter() {
        let ignore = lookup.square_bb(i);
        let potential_blockers =
            ray_between(king_square, i, lookup) & occupancy & !king_bb & !ignore;

        if potential_blockers.count_ones() == 1 {
            blockers |= potential_blockers;
        }
    }

    blockers
}

pub fn gen_pseudo_legal_moves(
    pos: &BoardState,
    list: &mut Vec<Move>,
    lookup: &Lookup,
    piece: PieceType,
) {
    let us = pos.active_player;
    let pieces = pos.bb(us, piece);
    let valid_pieces = pos.bb_for_color(!us);
    let empty_squares = !pos.bb_all();

    for (square, _) in pieces.iter() {
        let destinations = match piece {
            PieceType::King | PieceType::Knight => lookup.moves(square, piece),
            _ => lookup.sliding_moves(square, pos.bb_all(), piece),
        };
        let captures = destinations & valid_pieces;
        let quiets = destinations & empty_squares;

        extract_moves(square, captures, list, Capture);
        extract_moves(square, quiets, list, Quiet);
    }
}

pub fn gen_pseudo_legal_castles(pos: &BoardState, list: &mut Vec<Move>) {
    let us = pos.active_player;

    let (king_mask, queen_mask) = match us {
        Color::White => (96, 14),
        Color::Black => (6_917_529_027_641_081_856, 1_008_806_316_530_991_104),
    };

    let occupied = pos.bb_all();

    let (king_rights, queen_rights) = match us {
        Color::White => (
            pos.castling_rights.white_king,
            pos.castling_rights.white_queen,
        ),
        Color::Black => (
            pos.castling_rights.black_king,
            pos.castling_rights.black_queen,
        ),
    };

    if (occupied & king_mask == 0) && king_rights {
        let (to, from) = match us {
            Color::White => (G1 as u8, E1 as u8),
            Color::Black => (G8 as u8, E8 as u8),
        };
        let m = Move {
            to,
            from,
            kind: MoveType::CastleKing,
        };
        list.push(m);
    }

    if (occupied & queen_mask == 0) && queen_rights {
        let (to, from) = match us {
            Color::White => (C1 as u8, E1 as u8),
            Color::Black => (C8 as u8, E8 as u8),
        };
        let m = Move {
            to,
            from,
            kind: MoveType::CastleQueen,
        };
        list.push(m);
    }
}

pub fn king_square(pos: &BoardState) -> Square {
    let us = pos.active_player;
    pos.bb(us, PieceType::King).trailing_zeros() as Square
}

pub fn is_attacked(pos: &BoardState, square: Square, lookup: &Lookup) -> bool {
    let us = pos.active_player;

    if pawn_attacks(square, us) & pos.bb(!us, PieceType::Pawn) != 0 {
        return true;
    }

    let occupancies = pos.bb_all() & !pos.bb(us, PieceType::King);

    if lookup.sliding_moves(square, occupancies, PieceType::Rook)
        & (pos.bb(!us, PieceType::Rook) | pos.bb(!us, PieceType::Queen))
        != 0
    {
        return true;
    } else if lookup.sliding_moves(square, occupancies, PieceType::Bishop)
        & (pos.bb(!us, PieceType::Bishop) | pos.bb(!us, PieceType::Queen))
        != 0
    {
        return true;
    } else if lookup.moves(square, PieceType::Knight) & pos.bb(!us, PieceType::Knight) != 0 {
        return true;
    } else if lookup.moves(square, PieceType::King) & pos.bb(!us, PieceType::King) != 0 {
        return true;
    }

    false
}

pub fn extract_moves(from: u8, bb: Bitboard, list: &mut Vec<Move>, kind: MoveType) {
    for (square, _) in bb.iter() {
        let m = Move {
            to: square,
            from,
            kind,
        };
        list.push(m);
    }
}

pub fn knight_destinations(square: u8) -> Bitboard {
    let base_bb: Bitboard = 0;
    let base_bb = base_bb.add_at_square(square);

    let nnw = base_bb
        .checked_shl((NORTH + NORTH + WEST) as u32)
        .unwrap_or(0)
        & !FILEH;
    let nww = base_bb
        .checked_shl((NORTH + WEST + WEST) as u32)
        .unwrap_or(0)
        & !(FILEH | FILEG);
    let nne = base_bb
        .checked_shl((NORTH + NORTH + EAST) as u32)
        .unwrap_or(0)
        & !FILEA;
    let nee = base_bb
        .checked_shl((NORTH + EAST + EAST) as u32)
        .unwrap_or(0)
        & !(FILEA | FILEB);

    let sww = base_bb
        .checked_shr(-(SOUTH + WEST + WEST) as u32)
        .unwrap_or(0)
        & !(FILEG | FILEH);
    let ssw = base_bb
        .checked_shr(-(SOUTH + SOUTH + WEST) as u32)
        .unwrap_or(0)
        & !FILEH;
    let sse = base_bb
        .checked_shr(-(SOUTH + SOUTH + EAST) as u32)
        .unwrap_or(0)
        & !FILEA;
    let see = base_bb
        .checked_shr(-(SOUTH + EAST + EAST) as u32)
        .unwrap_or(0)
        & !(FILEA | FILEB);

    nnw | nww | nne | nee | sww | ssw | sse | see
}
#[derive(Copy, Clone)]
struct PawnDirections {
    rank7: Bitboard,
    rank3: Bitboard,
    north: i8,
}

/// Generate all pseudo-legal moves for the given position and add them
/// to the provided vector. Pseudo-legal moves are defined as a subset of
/// all legal moves for a given position which might also leave the king in check.
pub fn gen_pseudo_legal_pawn_moves(pos: &BoardState, list: &mut Vec<Move>) {
    let dirs = PawnDirections::new(pos.active_player);
    let pawns = pos.bb(pos.active_player, PieceType::Pawn);
    gen_quiet_pushes(pos, list, dirs, pawns);
    gen_captures(pos, list, dirs, pawns);
    gen_en_passant(pos, list, dirs, pawns);
    gen_promotions(pos, list, dirs, pawns);
}

/// Generate all quiet pushes, defined as single and double pushes,
/// but excludes all promotions.
fn gen_quiet_pushes(pos: &BoardState, list: &mut Vec<Move>, dirs: PawnDirections, pawns: Bitboard) {
    let pawns = pawns & !dirs.rank7;
    let empty_squares = !pos.bb_all();
    let single = pawns.shift(dirs.north) & empty_squares;

    let pawns = single & dirs.rank3;
    let empty_squares = !pos.bb_all();
    let double = pawns.shift(dirs.north) & empty_squares;

    extract_pawn_moves(single, dirs.north, Quiet, list);
    extract_pawn_moves(double, dirs.north + dirs.north, Quiet, list);
}

/// Generate all captures, excluding en passant captures and those which
/// result in promotions and under-promotions.
fn gen_captures(pos: &BoardState, list: &mut Vec<Move>, dirs: PawnDirections, pawns: Bitboard) {
    let us = pos.active_player;
    let pawns = pawns & !dirs.rank7;
    let their_king = pos.bb(!us, PieceType::King);
    let valid_pieces = pos.bb_for_color(!us) & !their_king;

    let left_captures = pawns.shift(dirs.north + WEST) & valid_pieces;
    let right_captures = pawns.shift(dirs.north + EAST) & valid_pieces;

    extract_pawn_moves(left_captures, dirs.north + WEST, Capture, list);
    extract_pawn_moves(right_captures, dirs.north + EAST, Capture, list);
}

/// Generate all en passant captures for the given position.
fn gen_en_passant(pos: &BoardState, list: &mut Vec<Move>, dirs: PawnDirections, pawns: Bitboard) {
    if pos.en_passant.is_none() {
        return;
    }

    let en_passant = en_passant_bb(pos);

    let left_captures = pawns.shift(dirs.north + WEST) & en_passant;
    let right_captures = pawns.shift(dirs.north + EAST) & en_passant;

    extract_pawn_moves(left_captures, dirs.north + WEST, EnPassantCapture, list);
    extract_pawn_moves(right_captures, dirs.north + EAST, EnPassantCapture, list);
}

/// Generate all promotions and under promotions, including pushes and captures on the eighth rank.
fn gen_promotions(pos: &BoardState, list: &mut Vec<Move>, dirs: PawnDirections, pawns: Bitboard) {
    let us = pos.active_player;
    let pawns = pawns & dirs.rank7;
    let empty_squares = !pos.bb_all();
    let their_king = pos.bb(!us, PieceType::King);
    let valid_captures = pos.bb_for_color(!us) & !their_king;

    let pushes = pawns.shift(dirs.north) & empty_squares;
    let left_captures = pawns.shift(dirs.north + WEST) & valid_captures;
    let right_captures = pawns.shift(dirs.north + EAST) & valid_captures;

    extract_promotions(pushes, dirs.north, list, PromotionType::Push);
    extract_promotions(
        left_captures,
        dirs.north + WEST,
        list,
        PromotionType::Capture,
    );
    extract_promotions(
        right_captures,
        dirs.north + EAST,
        list,
        PromotionType::Capture,
    );
}

/// Given a resulting bitboard and a relevant offset, find all pawn moves using the given offset.
pub fn extract_pawn_moves(bitboard: Bitboard, offset: i8, kind: MoveType, moves: &mut Vec<Move>) {
    for (square, _) in bitboard.iter() {
        let m = Move {
            to: square as u8,
            from: (square as i8 - offset) as u8,
            kind,
        };
        moves.push(m);
    }
}

/// Returns a bitboard representing all pawn attacks from the given square for the given color
pub fn pawn_attacks(square: Square, color: Color) -> Bitboard {
    let b: Bitboard = 0;
    let b = b.add_at_square(square);
    match color {
        Color::White => b.shift(NORTH + WEST) | b.shift(NORTH + EAST),
        Color::Black => b.shift(SOUTH + WEST) | b.shift(SOUTH + EAST),
    }
}

/// Given a resulting bitboard, find and enumerate all possible promotions using the provided offset.
fn extract_promotions(bitboard: Bitboard, offset: i8, moves: &mut Vec<Move>, kind: PromotionType) {
    for (square, _) in bitboard.iter() {
        let itr = match kind {
            PromotionType::Push => MoveType::promotion_itr(),
            PromotionType::Capture => MoveType::promotion_capture_itr(),
        };
        for promotion in itr {
            let m = Move {
                to: square as u8,
                from: (square as i8 - offset) as u8,
                kind: *promotion,
            };
            moves.push(m)
        }
    }
}

/// Given a game position, return a Bitboard that includes a non-zero bit only on the target en passant square.
fn en_passant_bb(pos: &BoardState) -> Bitboard {
    let square = pos.en_passant.unwrap_or(0);
    if square == 0 {
        0
    } else {
        Bitboard::for_square(square)
    }
}

impl PawnDirections {
    fn new(color: Color) -> PawnDirections {
        let rank7 = match color {
            Color::White => RANK7,
            Color::Black => RANK2,
        };
        let rank3 = match color {
            Color::White => RANK3,
            Color::Black => RANK6,
        };
        let north = match color {
            Color::White => NORTH,
            Color::Black => SOUTH,
        };
        PawnDirections {
            rank7,
            rank3,
            north,
        }
    }
}

#[allow(dead_code)]
pub fn debug_print(pos: &BoardState) -> String {
    let mut s = String::with_capacity(64);
    for i in 0..8 {
        for j in 0..8 {
            let file = j;
            let rank = 7 - i;
            let square = rank_file_to_index(rank, file);
            let piece = pos.type_on(square);
            let mut c;
            if piece.is_none() {
                c = '.';
            } else {
                c = match piece.unwrap() {
                    PieceType::Pawn => 'p',
                    PieceType::Rook => 'r',
                    PieceType::Knight => 'n',
                    PieceType::Bishop => 'b',
                    PieceType::King => 'k',
                    PieceType::Queen => 'q',
                };
                if pos.color_on(square).unwrap() == Color::White {
                    c = c.to_ascii_uppercase();
                }
            }
            print!("{}", c);
            s.push(c);
        }
        println!();
    }
    println!("{}", s);
    s
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bitboard::RANK2;
    use crate::board::BoardState;
    use crate::chess_move::Move;
    use crate::chess_move::MoveType::Quiet;
    use crate::fen::parse_fen;
    use crate::magic::{GenerationScheme, MagicRandomizer};
    use crate::move_gen::{gen_pseudo_legal_castles, king_square, MoveGenerator};
    use crate::square::SquareIndex;
    use crate::square::SquareIndex::{
        A1, A2, A3, B1, B2, B4, B5, C2, C3, C4, C5, C6, C8, D2, D3, D4, D5, E1, E2, E6, E7, E8, F1,
        F2, F3, F5, F6, G1, G2, G5, G8, H1, H2, H4,
    };

    #[test]
    #[ignore]
    fn perft_starting_position() {
        let mut pos = BoardState::default();
        let gen = MoveGenerator::new();
        let depth_1 = gen.perft(&mut pos, 1);
        let depth_2 = gen.perft(&mut pos, 2);
        let depth_3 = gen.perft(&mut pos, 3);
        let depth_4 = gen.perft(&mut pos, 4);
        let _depth_5 = gen.perft(&mut pos, 5);

        assert_eq!(depth_1, 20);
        assert_eq!(depth_2, 400);
        assert_eq!(depth_3, 8902);
        assert_eq!(depth_4, 197_281);
    }
    #[test]
    #[ignore]
    fn perft_kiwipete() {
        let mut pos =
            parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
                .unwrap();
        let gen = MoveGenerator::new();
        let depth_1 = gen.perft(&mut pos, 1);
        let depth_2 = gen.perft(&mut pos, 2);
        let depth_3 = gen.perft(&mut pos, 3);
        let depth_4 = gen.perft(&mut pos, 4);

        assert_eq!(depth_1, 48);
        assert_eq!(depth_2, 2039);
        assert_eq!(depth_3, 97862);
        assert_eq!(depth_4, 4_085_603);
    }

    #[test]
    #[ignore]
    fn perft_fen_3() {
        let mut pos = parse_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();

        let gen = MoveGenerator::new();
        let depth_1 = gen.perft(&mut pos, 1);
        let depth_2 = gen.perft(&mut pos, 2);
        let depth_3 = gen.perft(&mut pos, 3);
        let depth_4 = gen.perft(&mut pos, 4);
        let depth_5 = gen.perft(&mut pos, 5);

        assert_eq!(depth_1, 14);
        assert_eq!(depth_2, 191);
        assert_eq!(depth_3, 2812);
        assert_eq!(depth_4, 43238);
        assert_eq!(depth_5, 674_624);
    }

    #[test]
    #[ignore]
    fn perft_fen_4() {
        let mut pos =
            parse_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap();

        let gen = MoveGenerator::new();
        let depth_1 = gen.perft(&mut pos, 1);
        let depth_2 = gen.perft(&mut pos, 2);
        let depth_3 = gen.perft(&mut pos, 3);
        let depth_4 = gen.perft(&mut pos, 4);

        assert_eq!(depth_1, 6);
        assert_eq!(depth_2, 264);
        assert_eq!(depth_3, 9467);
        assert_eq!(depth_4, 422_333);
    }

    #[test]
    #[ignore]
    fn perft_fen_5() {
        let mut pos =
            parse_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();

        let gen = MoveGenerator::new();
        let depth_1 = gen.perft(&mut pos, 1);
        let depth_2 = gen.perft(&mut pos, 2);
        let depth_3 = gen.perft(&mut pos, 3);
        let depth_4 = gen.perft(&mut pos, 4);

        assert_eq!(depth_1, 44);
        assert_eq!(depth_2, 1486);
        assert_eq!(depth_3, 62379);
        assert_eq!(depth_4, 2_103_487);
    }

    #[test]
    #[ignore]
    fn perft_fen_6() {
        let mut pos =
            parse_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10")
                .unwrap();

        let gen = MoveGenerator::new();
        let depth_1 = gen.perft(&mut pos, 1);
        let depth_2 = gen.perft(&mut pos, 2);
        let depth_3 = gen.perft(&mut pos, 3);

        assert_eq!(depth_1, 46);
        assert_eq!(depth_2, 2079);
        assert_eq!(depth_3, 89890);
    }

    #[test]
    #[ignore]
    fn perft_fen_random() {
        let mut pos =
            parse_fen("r6r/1bp2pP1/R2qkn2/1P6/1pPQ4/1B3N2/1B1P2p1/4K2R b KQ c3 0 1").unwrap();

        let gen = MoveGenerator::new();
        let depth_1 = gen.perft(&mut pos, 1);
        let depth_2 = gen.perft(&mut pos, 2);
        let depth_3 = gen.perft(&mut pos, 3);

        assert_eq!(depth_1, 51);
        assert_eq!(depth_2, 2778);
        assert_eq!(depth_3, 111_425);
    }

    #[test]
    fn calculates_blockers() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen("8/8/2r5/5b2/2P5/2P5/2K1Pr2/8 w - - 0 1").unwrap();
        let king_square = king_square(&pos);

        assert_eq!(calculate_blockers(&pos, &lookup, king_square), 4096);
    }

    fn make_move(to: SquareIndex, from: SquareIndex) -> Move {
        Move {
            to: to as u8,
            from: from as u8,
            kind: Quiet,
        }
    }

    #[test]
    fn moves_between_same_rank() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let b = ray_between(A1 as u8, H1 as u8, &lookup);

        assert_eq!(b, 255);
    }

    #[test]
    fn moves_along_diagonal() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let b = ray_between(B4 as u8, E7 as u8, &lookup);

        assert_eq!(b, 4_512_412_933_816_320);
    }

    #[test]
    fn cannot_capture_checking_piece_while_pinned() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen("2r5/8/8/2B5/8/8/8/2K3r1 w - - 0 1").unwrap();

        let mv = make_move(G1, C5);

        let king_square = king_square(&pos);
        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        assert_eq!(
            is_legal(&pos, &mv, &lookup, blockers, checkers, king_square),
            false
        );
    }

    #[test]
    fn cannot_block_checking_piece_while_pinned() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen("2r5/8/8/2B5/8/8/8/2K4r w - - 0 1").unwrap();

        let king_square = king_square(&pos);
        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        let mv = make_move(G1, C5);
        assert_eq!(
            is_legal(&pos, &mv, &lookup, blockers, checkers, king_square),
            false
        );
    }

    #[test]
    fn cannot_move_pinned_piece() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen("8/8/8/8/1K1N3r/8/8/8 w - - 0 1").unwrap();

        let king_square = king_square(&pos);
        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        let mv = make_move(C6, D4);
        assert_eq!(
            is_legal_non_king_move(&pos, &mv, &lookup, blockers, checkers, king_square),
            false
        );

        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        let mv = make_move(C2, D4);
        assert_eq!(
            is_legal_non_king_move(&pos, &mv, &lookup, blockers, checkers, king_square),
            false
        );
    }

    #[test]
    fn can_move_piece_along_pinned_ray() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen("8/8/8/8/8/8/1K3R1r/8 w - - 0 1").unwrap();

        let king_square = king_square(&pos);
        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        // Move towards pinner without capture
        let mv = make_move(G2, F2);
        assert_eq!(
            is_legal_non_king_move(&pos, &mv, &lookup, blockers, checkers, king_square),
            true
        );

        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        // Move towards pinner with capture
        let mv = make_move(H2, F2);
        assert_eq!(
            is_legal_non_king_move(&pos, &mv, &lookup, blockers, checkers, king_square),
            true
        );

        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        // Move away from pinner
        let mv = make_move(E2, F2);
        assert_eq!(
            is_legal_non_king_move(&pos, &mv, &lookup, blockers, checkers, king_square),
            true
        );

        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        // Moving off pin is illegal
        let mv = make_move(F1, F2);
        assert_eq!(
            is_legal_non_king_move(&pos, &mv, &lookup, blockers, checkers, king_square),
            false
        );
    }

    #[test]
    fn cannot_move_non_king_with_multiple_checkers() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen("8/1r6/8/8/3N4/8/1K5r/8 w - - 0 1").unwrap();

        let king_square = king_square(&pos);
        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        let mv = make_move(D4, C6);
        assert_eq!(
            is_legal_non_king_move(&pos, &mv, &lookup, blockers, checkers, king_square),
            false
        );
    }

    #[test]
    fn can_move_king() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen("8/8/8/8/8/8/1K5r/8 w - - 0 1").unwrap();

        let mv = make_move(A2, B2);
        assert_eq!(is_legal_king_move(&pos, &mv, &lookup), false);

        let mv = make_move(B1, B2);
        assert_eq!(is_legal_king_move(&pos, &mv, &lookup), true);
    }

    #[test]
    fn cannot_block_using_xray() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen("8/8/8/8/8/3B4/3K3r/8 w - - 0 1").unwrap();

        let king_square = king_square(&pos);
        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        let mv = make_move(C2, D3);
        assert_eq!(
            is_legal_non_king_move(&pos, &mv, &lookup, blockers, checkers, king_square),
            false
        );

        let mv = make_move(E2, D3);
        assert_eq!(
            is_legal_non_king_move(&pos, &mv, &lookup, blockers, checkers, king_square),
            true
        );
    }

    #[test]
    fn king_cannot_castle_through_check() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen("8/8/8/8/8/3b4/8/R3K2R w KQ - 0 1").unwrap();
        let _mv = make_move(C2, D3);
        let mv = Move {
            to: 0,
            from: 0,
            kind: MoveType::CastleKing,
        };
        assert_eq!(is_legal_castle(&pos, &mv, &lookup, 0), false);
    }

    #[test]
    fn king_cannot_castle_in_check() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen("8/8/8/8/8/2b5/8/R3K2R w KQ - 0 1").unwrap();
        let mv = Move {
            to: 0,
            from: 0,
            kind: MoveType::CastleKing,
        };
        assert_eq!(is_legal_castle(&pos, &mv, &lookup, 1), false);
    }

    #[test]
    fn en_passant_discovered_check() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen("8/8/8/K2Pp2q/8/8/8/8 w - e6 0 1").unwrap();
        let mv = Move {
            to: E6 as u8,
            from: D5 as u8,
            kind: MoveType::EnPassantCapture,
        };

        let king_square = king_square(&pos);
        let _blockers = calculate_blockers(&pos, &lookup, king_square);

        assert_eq!(is_legal_en_passant(&pos, &mv, &lookup, king_square), false);
    }

    #[test]
    fn en_passant_out_of_check() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen("8/8/8/3Pp2q/3K4/8/8/8 w - e6 0 1").unwrap();
        let mv = Move {
            to: E6 as u8,
            from: D5 as u8,
            kind: MoveType::EnPassantCapture,
        };

        let king_square = king_square(&pos);
        let _blockers = calculate_blockers(&pos, &lookup, king_square);

        assert_eq!(is_legal_en_passant(&pos, &mv, &lookup, king_square), true);
    }

    #[test]
    fn random_fen_1() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen("8/2p5/3p4/KP5r/5R1k/8/4P1P1/8 b - - 0 1").unwrap();
        let mv = Move {
            to: G5 as u8,
            from: H4 as u8,
            kind: MoveType::Quiet,
        };

        let king_square = king_square(&pos);
        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        assert_eq!(
            is_legal(&pos, &mv, &lookup, blockers, checkers, king_square),
            true
        );
    }

    #[test]
    fn random_fen_2() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos =
            parse_fen("rnbqk1nr/pppp1ppp/8/4p3/1b1P4/P7/1PP1PPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let mv = Move {
            to: B4 as u8,
            from: A3 as u8,
            kind: MoveType::Capture,
        };

        let king_square = king_square(&pos);
        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        assert_eq!(
            is_legal(&pos, &mv, &lookup, blockers, checkers, king_square),
            true
        );
    }

    #[test]
    fn random_fen_3() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos =
            parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/P1N2Q1p/1PPBBPPP/R3K2R w KQkq - 0 1")
                .unwrap();
        let mv = Move {
            to: A3 as u8,
            from: B4 as u8,
            kind: MoveType::Capture,
        };

        let king_square = king_square(&pos);
        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        assert_eq!(
            is_legal(&pos, &mv, &lookup, blockers, checkers, king_square),
            true
        );
    }

    #[test]
    fn random_fen_4() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos =
            parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/Pp2P3/2N2Q1p/1PPBBPPP/R3K2R w KQkq a3 0 1")
                .unwrap();
        let mv = Move {
            to: A3 as u8,
            from: B4 as u8,
            kind: MoveType::EnPassantCapture,
        };

        let king_square = king_square(&pos);
        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        assert_eq!(
            is_legal(&pos, &mv, &lookup, blockers, checkers, king_square),
            true
        );
    }

    #[test]
    fn castle_through_knight_attacks() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen("r3k2r/p1ppqpb1/bnN1pnp1/3P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1")
            .unwrap();
        let mv = Move {
            to: C8 as u8,
            from: E8 as u8,
            kind: MoveType::CastleQueen,
        };

        let king_square = king_square(&pos);
        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        assert_eq!(
            is_legal(&pos, &mv, &lookup, blockers, checkers, king_square),
            false
        );
    }

    #[test]
    fn castle_through_more_knight_attacks() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen("r3k2r/p1ppqpb1/bn2pnN1/3P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1")
            .unwrap();
        let mv = Move {
            to: G8 as u8,
            from: E8 as u8,
            kind: MoveType::CastleKing,
        };

        let king_square = king_square(&pos);
        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        assert_eq!(
            is_legal(&pos, &mv, &lookup, blockers, checkers, king_square),
            false
        );
    }

    #[test]
    fn castle_through_even_more_knight_attacks() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen("r3k2r/p1ppqNb1/bn2pn2/3P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1")
            .unwrap();
        let mv = Move {
            to: C8 as u8,
            from: E8 as u8,
            kind: MoveType::CastleQueen,
        };

        let king_square = king_square(&pos);
        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        assert_eq!(
            is_legal(&pos, &mv, &lookup, blockers, checkers, king_square),
            false
        );
    }

    #[test]
    fn queen_captures() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen("r3k2r/p1ppqpb1/1n2pnp1/3PN3/1p2P3/2N2Q1p/PPPBbPPP/R2K3R w KQkq - 0 1")
            .unwrap();
        let mv = Move {
            to: E2 as u8,
            from: F3 as u8,
            kind: MoveType::Capture,
        };

        let king_square = king_square(&pos);
        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        assert_eq!(
            is_legal(&pos, &mv, &lookup, blockers, checkers, king_square),
            true
        );
    }

    #[test]
    fn capture_checker_behind_ray() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos =
            parse_fen("r3k2r/p1pp1pb1/bn2pnp1/1B1PN3/1pq1P3/2N2Q1p/PPPB1PPP/R4K1R w kq - 4 3")
                .unwrap();
        let mv = Move {
            to: C4 as u8,
            from: B5 as u8,
            kind: MoveType::Capture,
        };

        let king_square = king_square(&pos);
        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        assert_eq!(
            is_legal(&pos, &mv, &lookup, blockers, checkers, king_square),
            true
        );
    }

    #[test]
    fn challenge() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen("r6r/1bp2pP1/R2qkn2/1P6/1pPQ4/1B3N2/1B1P2p1/4K2R b K c3 0 1").unwrap();
        let mv = Move {
            to: C3 as u8,
            from: B4 as u8,
            kind: MoveType::EnPassantCapture,
        };

        let king_square = king_square(&pos);
        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        assert_eq!(
            is_legal(&pos, &mv, &lookup, blockers, checkers, king_square),
            false
        );
    }

    #[test]
    fn castle_pawn_attacks() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen("8/8/8/8/8/8/6p1/4K2R w K - 0 1").unwrap();
        let mv = Move {
            to: E1 as u8,
            from: G1 as u8,
            kind: MoveType::CastleKing,
        };

        let king_square = king_square(&pos);
        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        assert_eq!(
            is_legal(&pos, &mv, &lookup, blockers, checkers, king_square),
            false
        );
    }

    #[test]
    fn captures_attacker_on_ray() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen("8/8/8/8/8/8/1K1R2r1/8 w - - 0 1").unwrap();
        let mv = Move {
            to: G2 as u8,
            from: D2 as u8,
            kind: MoveType::Capture,
        };

        let king_square = king_square(&pos);
        let blockers = calculate_blockers(&pos, &lookup, king_square);
        let checkers = attacks_to(&pos, king_square, &lookup);

        assert_eq!(
            is_legal(&pos, &mv, &lookup, blockers, checkers, king_square),
            true
        );
    }

    #[test]
    fn castles_no_obstruction() {
        let pos = parse_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1").unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_castles(&pos, &mut list);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn no_castles_with_obstruction() {
        let pos = parse_fen("8/8/8/8/8/8/8/R3KB1R w KQ - 0 1").unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_castles(&pos, &mut list);
        assert_eq!(list.len(), 1);

        let pos = parse_fen("8/8/8/8/8/8/8/R1B1K2R w KQ - 0 1").unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_castles(&pos, &mut list);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn no_castles_without_rights() {
        let pos = parse_fen("8/8/8/8/8/8/8/R3K2R w K - 0 1").unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_castles(&pos, &mut list);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn black_queenside_castle() {
        let pos = parse_fen("r3k2r/p1ppq1b1/bn2pn2/3P2N1/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 2")
            .unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_castles(&pos, &mut list);
        let _m1 = list.get(0).unwrap();
        let _m2 = list.get(1).unwrap();
        assert_eq!(list.len(), 2);
    }
    #[test]
    fn gen_random_pawn_moves1() {
        let pos = parse_fen("3N4/1p1N2R1/kp3PQp/8/p2P4/B7/6p1/b2b2K1 w - - 0 1").unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn gen_random_pawn_moves2() {
        let pos = parse_fen("8/1P5n/1NB5/2KbQ1P1/2n5/p4R2/Pp2p3/1k2b3 w - - 0 1").unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn gen_random_pawn_moves3() {
        let pos = parse_fen("3r2r1/P6b/q2pKPk1/4P3/1p1P1R2/5n2/1B2N3/8 w - - 0 1").unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 7);
    }

    #[test]
    fn gen_random_pawn_moves4() {
        let pos = parse_fen("8/4PP2/2n3p1/6P1/2p1p2q/K1P3k1/b1p1P1B1/2R5 w - - 0 1").unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 9);
    }

    #[test]
    fn gen_random_pawn_moves5() {
        let pos = parse_fen("3bBr2/8/P7/2PPp3/1N6/3bK2R/2Pp4/1k1qN3 w - d6 0 1").unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 7);
    }

    #[test]
    fn extract_basic_pawn_moves() {
        let b = RANK2;
        let mut moves: Vec<Move> = Vec::new();
        extract_pawn_moves(b, NORTH, Quiet, &mut moves);
        assert_eq!(moves.len(), 8);
        assert_eq!(moves.get(0).unwrap().to, 8);
        assert_eq!(moves.get(1).unwrap().to, 9);
    }

    /// Pawns moves for FEN
    /// 3N4/1p1N2R1/kp3PQp/8/p2P4/B7/6p1/b2b2K1 w - - 0 1
    #[test]
    fn extract_random_pawns() {
        let b: Bitboard = 35_184_506_306_560;
        let mut moves: Vec<Move> = Vec::new();
        extract_pawn_moves(b, NORTH, Quiet, &mut moves);
        assert_eq!(moves.len(), 2);
        assert_eq!(moves.get(0).unwrap().to, D4 as u8);
        assert_eq!(moves.get(0).unwrap().from, D3 as u8);
        assert_eq!(moves.get(1).unwrap().to, F6 as u8);
        assert_eq!(moves.get(1).unwrap().from, F5 as u8);
    }

    #[test]
    fn gen_en_passant() {
        let pos = parse_fen("8/8/3p4/KPp4r/5R1k/8/8/8 w - c6 0 1").unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn gen_a3_to_b4() {
        let pos = parse_fen("8/8/8/8/1p6/P7/8/8 w - - 0 1").unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn gen_b4_to_a3() {
        let mut pos = parse_fen("8/8/8/8/Pp6/8/8/8 b - a3 0 1").unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 2);
        let mv = *list.get(1).unwrap();
        pos.make_move(mv);
        assert_eq!(pos.bb_all(), 65536)
    }
}
