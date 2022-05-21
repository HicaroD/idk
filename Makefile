CXX = g++

run: src/main.cpp src/idk/* ;
	g++ -I src/ src/main.cpp src/lexer.cpp
