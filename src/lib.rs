//! A simple chess engine and move generator.
//!
//! Purple is a high-level API, meaning that many of the finer details are hidden from clients.
//! For most functionality, clients will interact with the `Game` struct which manages
//! move generator, move evaluation and selection, and board state.
//!
//! Purple also includes a UCI (Universal Chess Interface) module for use with UCI programs
//! like ArenaChess and CuteChess.
//!
//! There is also a `purple` binary available, see [source](https://github.com/jlervin/purple).
//!
//! # Example
//! ```rust
//! use purple::Game;
//!
//! let mut game = Game::new(); // from the standard starting position
//! let moves = game.legal_moves();
//!
//! let best_move = game.best_move();
//! game.make_move(best_move.mv);
//! ```
//!

pub use crate::game::Game;

/// A struct representing the  state of the board.
pub mod board_state;

/// Tools for UCI interfacing
pub mod uci;

/// Common types and utilities such as pieces, moves, squares, etc.
pub mod common;

/// A struct representing the state of a game and move generation/selection infrastructure.
mod game;
/// Utilities for generating magic numbers
mod magic;

/// Utilities for generating moves from a given position
mod move_gen;

/// Utilities for searching and evaluation from a given a position
mod search;

/// A transposition table which optimizes searching by caching previous positions
mod table;
