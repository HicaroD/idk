CXX = g++
CXXFLAGS = -Wall -Werror -Wextra -Wpedantic
TARGET = idk

run: src/main.cpp src/idk/* ;
	g++ ${CXXFLAGS} -I src/ src/main.cpp src/lexer.cpp -o ${TARGET}
