use crate::{
    board_state::{board::BoardState, fen::parse_fen},
    common::{chess_move::Move, eval_move::EvaledMove, stats::Stats},
    move_gen::generator::{debug_print, MoveGenerator},
    search::{alpha_beta::AlphaBeta, search::Searcher},
};

/// A struct which encapsulates a chess game, which includes the ability to generate legal moves
/// and determine the best move from a given position.
/// ```rust
/// use purple::Game;
///
/// let game = Game::new();
/// let moves = game.legal_moves();
/// ```
/// Generating a new game is relatively expensive, as it requires creating lookup tables
/// for move generation.
pub struct Game {
    gen: MoveGenerator,
    pos: BoardState,
    searcher: AlphaBeta,
    // TODO: implement cached value for legal_moves
    // TODO: implement stack of previously chosen positions
}

impl Game {
    /// Construct a new game from the default starting position.
    pub fn new() -> Game {
        let gen = MoveGenerator::new();
        let pos = BoardState::default();
        let searcher = AlphaBeta::new();
        Game { gen, pos, searcher }
    }

    /// Construct a new game using the given FEN string.
    pub fn from_fen(fen: &str) -> Result<Game, String> {
        let gen = MoveGenerator::new();
        let pos = parse_fen(fen)?;
        let searcher = AlphaBeta::new();
        Ok(Game { gen, pos, searcher })
    }

    /// Using the current state of the game, return the move which is best
    /// for the active player along with it's evaluation.
    ///
    /// `best_move` uses a searcher which implements a transposition table.
    /// Note that the table *is not* cleared between runs automatically and must
    /// be manually reset if you need to do so.
    pub fn best_move(&mut self) -> EvaledMove {
        self.searcher.best_move(&mut self.pos)
    }

    /// Using the current state of the game, return the move which is best
    /// for the active player along with it's evaluation, only searching up to the maximum
    /// provided depth for typical evaluation.
    ///
    /// `best_move_depth` uses a searcher which implements a transposition table.
    /// Note that the table *is not* cleared between runs automatically and must
    /// be manually reset if you need to do so.
    pub fn best_move_depth(&mut self, depth: usize) -> EvaledMove {
        self.searcher.best_move_depth(&mut self.pos, depth)
    }

    /// Return a vector of all legal moves from the current position.
    pub fn legal_moves(&self) -> Vec<Move> {
        self.gen.all_moves(&self.pos)
    }

    /// Apply the given move to the game, returns an error if the given move is illegal.
    pub fn make_move(&mut self, mv: Move) -> Result<(), &'static str> {
        let legal_moves = self.legal_moves();
        if !legal_moves.contains(&mv) {
            return Err("Move is not legal in the position");
        }
        self.pos.make_move(mv);
        Ok(())
    }

    /// Runs a performance test of the Game's move generator, returning the total number
    /// of nodes calculated at the given depth.
    pub fn perft(&self, depth: usize) -> usize {
        self.gen.perft(&self.pos, depth)
    }

    /// Set whether or not the move searcher should use a transposition table to remember
    /// previously seen positions and their evaluations.
    pub fn use_table(&mut self, setting: bool) {
        self.searcher.use_table(setting);
    }

    /// Return a string representing the position, useful for debugging purposes.
    pub fn debug(&self) -> String {
        debug_print(&self.pos)
    }

    /// Return a stats struct, which contains metrics for the previous search
    pub fn stats(&self) -> &Stats {
        self.searcher.stats()
    }
}
