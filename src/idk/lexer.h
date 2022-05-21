#ifndef LEXER_H 
#define LEXER_H

#include <vector>

class Lexer {
    private:
	std::vector<char> source_code;
	char current_char;
	int position;

    public:
	Lexer(const std::vector<char>& source);

	char get_current_char();
	void advance();
	bool is_eof();
};

#endif
