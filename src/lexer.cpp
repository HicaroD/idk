#include <iostream>
#include <idk/lexer.h>
#include <cstdlib>
#include <cctype>
#include <vector>

Lexer::Lexer(const std::vector<char>& source) {
    source_code = source;
    current_char = '0';
    position = 0;
}

Token new_token(TokenType type, std::string id) {
    return Token { type, id };
}

Token classify_identifier(std::string identifier) {
    if(identifier.compare("def") == 0) {
	return new_token(TokenType::Identifier, identifier);

    } else if(identifier.compare("return") == 0) {
	return new_token(TokenType::Return, identifier);

    } else {
	return new_token(TokenType::Identifier, identifier);
    }
}

void Lexer::skip_whitespace() {
    while(isspace(current_char)) {
	advance();
    }
}

void Lexer::advance() {
    if(source_code.empty()) {
	exit(1);
    } else if(!is_eof()) {
	current_char = source_code[position++];
    }
}

bool Lexer::is_eof() {
    return position == int(source_code.size());
}

char Lexer::get_current_char() {
    return current_char;
}

std::string Lexer::get_identifier() {
    std::string identifier; 
    identifier += current_char;

    advance();
    while((isalnum(current_char) || current_char == '_') && !is_eof()) {
	identifier += current_char;
	advance();
    }
    return identifier;
}

std::vector<Token> Lexer::tokenize() {
    std::vector<Token> tokens;

    while(!is_eof()) {
	skip_whitespace();

	if(isalpha(current_char)) {
	    std::string identifier = get_identifier();
	    Token token = classify_identifier(identifier);
	    tokens.push_back(token);
	} 
	
	std::string token = "";
	token += current_char;

	if(current_char == '{' || current_char == '}') {
	    tokens.push_back(new_token(TokenType::CurlyBraces, token));

	} else if(current_char == '[' || current_char == ']') {
	    tokens.push_back(new_token(TokenType::Parenthesis, token));

	} else if(current_char == '=') {
	    tokens.push_back(new_token(TokenType::EqualSign, token));

	} else if(current_char == ':') {
	    tokens.push_back(new_token(TokenType::Colon, token));

	} else if(current_char == ';') {
	    tokens.push_back(new_token(TokenType::Semicolon, token));

	} else if(current_char == ',') {
	    tokens.push_back(new_token(TokenType::Comma, token));

	} else {
	    std::cerr << "Error: Invalid token \"" << current_char << "\"" << std::endl;
	    exit(1);
	}
    }

    return tokens;
}
