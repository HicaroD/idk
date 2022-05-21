CXX      = g++
INCLUDE  = src/idk/*
TARGET   = idk
SRC      = src/*.cpp
CXXFLAGS = -Wall -Werror -Wextra -Wpedantic -I src/ -o ${TARGET}

${TARGET}: ${INCLUDE} ${SRC} ;
	${CXX} ${CXXFLAGS} ${SRC}
