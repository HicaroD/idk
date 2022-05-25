#!/bin/bash

# TODO: Add command for formatting code (.h and .cpp) from "src" directory before compile
if [ -z $1 ] 
then
    echo You should pass a path to some source code
else
    make && ./idk $1
    rm idk
fi
