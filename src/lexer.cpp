#include <iostream>
#include <idk/lexer.h>
#include <cstdlib>
#include <vector>

Lexer::Lexer(const std::vector<char>& source) {
    source_code = source;
    current_char = '0';
    position = 0;
    eof = false;
}

void Lexer::advance() {
    if(source_code.empty()) {
	exit(1);
    } else if(position + 1 == int(source_code.size())) {
	set_eof();
    } else {
	current_char = source_code[position++];
    }
}

bool Lexer::is_eof() {
    return eof;
}

void Lexer::set_eof() {
    eof = true;
}

char Lexer::get_current_char() {
    return current_char;
}
