#!/bin/bash

if [ -z $1 ]
then
    echo "You should pass an input file"
else
    cargo fmt
    echo -e "\n-----RUNNING UNIT TESTS-----\n"
    cargo test
    echo -e "\n-----FINISHING UNIT TESTS-----\n"
    cargo run -- $1
fi
