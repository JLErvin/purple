#!/bin/bash

fen=$1

#for i in $(seq 8); do
    #./target/release/purple -a $i "$fen"
    let i=8
    ./target/release/purple -a $i "$fen"
    echo
#done
