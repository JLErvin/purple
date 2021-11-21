#!/bin/bash

fen=$1

for i in $(seq 8); do
    #./target/release/purple -a $i "$fen"
    ./target/release/purple -x $i "$fen"
    echo
done
