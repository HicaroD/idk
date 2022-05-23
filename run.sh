#!/bin/zsh

if [ -z $1 ] 
then
    echo "You should pass a path to some source code"
else
    make && ./idk $1
fi
