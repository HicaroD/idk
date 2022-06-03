#!/bin/bash

if [ -z $1 ]
then
    echo "You should pass an input file"
else
    cargo fmt
    cargo run -- $1
fi
