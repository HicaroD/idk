#!/bin/bash

format_with_clang() {
    clang-format -i --style Google $1
}

if [ -z $1 ] 
then
    echo You should pass a path to some source code
else
    format_with_clang src/*.cpp
    format_with_clang src/idk/*.h
    make && ./idk $1
    rm idk
fi
