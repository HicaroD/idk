#!/bin/bash

if [ -z $1 ]
then
    echo "You should pass an input file"
else
    cargo fmt
    cargo test
    cargo run -- --name $1 --target C
fi
