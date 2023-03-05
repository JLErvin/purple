<div align='center'>
    <h1>purple</h1>
</div>

Purple is a simple UCI-compliant chess engine and move generator written in Rust.
This is a hobbyist project with the goal of learning, and documenting, the technologies
that go into modern chess engines.
An in-depth blog on the development of `purple` is available on the author's website

* [Part 1: Writing a chess engine move generator](https://www.josherv.in/2021/03/19/chess-1/)
* [Part 2: Writing a chess engine move selector](https://www.josherv.in/2022/12/17/chess-2/)

## Installation
To use the `purple` binary, you can always build from source. Simply clone this repository and execute
```bash
$ cargo build --release
```

This will build an optimized binary in `target/releases/purple`.

## Features

* Board representation using bitboards
* Compute magic numbers during application startup
* Sliding move generation using magic numbers
* Standalone legal-move generator
* Move searching using alpha-beta pruning
* Transpositon tables using Zobrist hashing
* Quiescence search
* Internal Iterative Deepending
* Late Move Reduction
* TT Move Ordering
* MVV-LVA Move Ordering
* Both a high-level API and binary are available in this repo
* UCI compliance

## Planned features

* Parallel Searching
* Null Move Pruning
* 3-Fold Repition Detection

## Usage

```
purple 
Joshua L Ervin
A UCI chess engine

USAGE:
    purple [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --alpha-perft <depth> <fen>    run a performance test on the alpha-beta searcher
    -m, --mini-perft <depth> <fen>     run a performance test on the minimax searcher
    -p, --perft <depth> <fen>          run a performance test on the move generator
```
