#include <iostream>
#include <cstdlib>
#include <fstream>
#include <string>

#include "idk/lexer.h"
#include "idk/parser.h"

std::vector<char> read_source_code(std::string filename) {
    std::vector<char> source_code;
    std::ifstream file(filename);

    if(!file.is_open()) {
	std::cerr << "Error: No such file" << std::endl;
	std::exit(1);
    }

    char current_char;
    while(file >> std::noskipws >> current_char) {
	source_code.push_back(current_char);
    }

    return source_code;
}

int main(int argc, char **argv) {
    if(argc < 2) {
	std::cerr << "Error: No input file" << std::endl;
	exit(1);
    }

    std::string filename = argv[1];
    std::vector<char> source_code = read_source_code(filename);

    if(source_code.empty()) {
	exit(1);
    }

    std::cout << "----STARTING LEXER----" << std::endl;
    Lexer* lexer = new Lexer(source_code);
    lexer->advance();

    std::vector<Token> tokens = lexer->tokenize();
    for(Token token: tokens) {
        printf("[%s]\n", token.id.c_str());
    }
    std::cout << "----ENDING LEXER----\n" << std::endl;

    std::cout << "----STARTING PARSER----" << std::endl;
    Parser* parser = new Parser(tokens);
    parser->generate_ast();
    std::cout << "----ENDING PARSER----" << std::endl;

    delete lexer;
    delete parser;
    return 0;
}
