#![warn(clippy::pedantic)]

//! A simple chess engine and move generator.
//!
//! Purple is a high-level API, meaning that many of the finer details are hidden from clients.
//! For most functionality, clients will interact with the `Game` struct which manages
//! move generator, move evaluation and selection, and board state.
//!
//! Purple also includes a UCI (Universal Chess Interface) module for use with UCI programs
//! like `ArenaChess` and `CuteChess`.
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

mod bitboard;
mod board;
mod chess_move;
mod fen;
mod game;
mod magic;
mod move_gen;
mod piece;
mod search;
mod square;
mod table;
pub mod uci;
