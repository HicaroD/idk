CXX      = g++
INCLUDE  = src/idk/*
TARGET   = idk
SRC      = src/*.cpp
CXXFLAGS = -std=c++20 -Wall -Werror -Wextra -Wpedantic -I src/ -o ${TARGET}

${TARGET}: ${INCLUDE} ${SRC} ;
	${CXX} ${CXXFLAGS} ${SRC}
