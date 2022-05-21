#include <iostream>
#include <cstdlib>
#include <fstream>
#include <string>
#include <idk/lexer.h>

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
	return 1;
    }

    std::string filename = argv[1];
    std::vector<char> source_code = read_source_code(filename);

    Lexer* lexer = new Lexer(source_code);

    while(!lexer->is_eof()) {
	std::cout << lexer->get_current_char() << std::endl;
	lexer->advance();
    }

    delete lexer;
    return 0;
}
