CXX = g++
CXXFLAGS = -Wall -Werror -Wextra -Wpedantic -I src/
TARGET = idk
SRC    = src/*.cpp

run: src/main.cpp src/idk/* ;
	g++ ${CXXFLAGS} ${SRC} -o ${TARGET}
