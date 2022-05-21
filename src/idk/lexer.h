#ifndef LEXER_H 
#define LEXER_H

#include <vector>
#include <string>

enum class TokenType {
    // Keywords
    Def,
    Return,

    Identifier,

    // Special characters
    Parenthesis,
    CurlyBraces,
    EqualSign,
    Semicolon,
    Colon,
    Comma,

    // Operators
    Plus,
    Minus,
};

struct Token {
    TokenType type;
    std::string id;
};

Token new_token(TokenType type, std::string id);
Token classify_identifier(std::string identifier);

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
	void skip_whitespace();
	std::vector<Token> tokenize();
	std::string get_identifier();
};

#endif
