#!/bin/bash
#
#

cargo build --release && printf "position fen r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1\ngo movetime 1000\nquit" | ./target/release/purple
#cargo build --release && printf "position startpos\ngo movetime 10000\nquit" | ./target/release/purple
