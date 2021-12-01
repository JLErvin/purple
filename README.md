<div align='center'>
    <h1>purple</h1><br>
</div>

Purple is a simple UCI-compliant chess engine and move generator written in Rust.
It aims to be performant while maintaining a readable and understandable codebase.

## Features

* Board representation using bitboards
* Ability to quickly compute magic numbers during application startup
* Sliding move generation using magic numbers
* Standalone legal-move generator
* Move searching using alpha-beta pruning
* UCI compliance

## Planned features

* Move searching using transposition tables with Zobirst hashing
* Iterative deepening with Jamboree searching

## Usage

```
USAGE:
    purple [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --alpha-perft <depth> <fen>    run a performance test on the alpha-beta searcher
    -p, --perft <depth> <fen>          run a performance test on the move generator
```
