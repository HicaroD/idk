#include <iostream>
#include <idk/lexer.h>
#include <cstdlib>
#include <vector>

Lexer::Lexer(const std::vector<char>& source) {
    source_code = source;
    current_char = '0';
    position = 0;
}

void Lexer::advance() {
    if(source_code.empty()) {
	exit(1);
    } else if(!is_eof()) {
	current_char = source_code[position++];
    }
}

bool Lexer::is_eof() {
    return position + 1 == int(source_code.size());
}

char Lexer::get_current_char() {
    return current_char;
}
