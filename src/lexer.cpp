#include <iostream>
#include <cstdlib>
#include <cctype>
#include <vector>

#include "idk/lexer.h"

Lexer::Lexer(const std::vector<char>& source) {
    source_code = source;
    current_char = '0';
    position = 0;
}

Token new_token(TokenType type, std::string id) { return Token { type, id }; }

Token classify_identifier(std::string identifier) {
    if(identifier.compare("def") == 0) {
	return new_token(TokenType::Def, identifier);

    } else if(identifier.compare("return") == 0) {
	return new_token(TokenType::Return, identifier);

    } else if(identifier.compare("if") == 0) {
	return new_token(TokenType::If, identifier);

    } else if(identifier.compare("elif") == 0) {
	return new_token(TokenType::Elif, identifier);

    } else if(identifier.compare("else") == 0) {
	return new_token(TokenType::Else, identifier);
    }

    return new_token(TokenType::Identifier, identifier);
}

void Lexer::skip_whitespace() {
    while(isspace(current_char)) {
	advance();
    }
}

void Lexer::advance() {
    if(!is_eof()) {
	current_char = source_code[position++];
    }
}

bool Lexer::is_eof() { return position == int(source_code.size()); }

std::string Lexer::get_number() {
    std::string number;
    number += current_char;
    advance();

    while(isdigit(current_char) || current_char == '.') {
	number += current_char;
	advance();
    }

    return number;
}

std::string Lexer::get_identifier() {
    std::string identifier; 
    identifier += current_char;

    advance();
    while((isalnum(current_char) || current_char == '_')) {
	identifier += current_char;
	advance();
    }
    return identifier;
}

void Lexer::consume(Token token, std::vector<Token>& tokens) {
    tokens.push_back(token);
    advance();
}

std::vector<Token> Lexer::tokenize() {
    std::vector<Token> tokens;

    while(!is_eof()) {
	skip_whitespace();

	std::string token;
	token = current_char;

	std::cout << "Current char: " << current_char << std::endl;

	if(isalpha(current_char)) {
	    std::string identifier = get_identifier();
	    Token token = classify_identifier(identifier);
	    tokens.push_back(token);

	} else if(isdigit(current_char)) {
	    std::string number = get_number();
	    tokens.push_back(new_token(TokenType::Number, number));

	} else {
	    switch(current_char) {
		case '(':
		case ')':
		    consume(new_token(TokenType::Parenthesis, token), tokens);
		    break;

		case '{':
		case '}':
		    consume(new_token(TokenType::CurlyBraces, token), tokens);
		    break;

		case '[':
		case ']':
		    consume(new_token(TokenType::Brackets, token), tokens);
		    break;

		case '=':
		    consume(new_token(TokenType::EqualSign, token), tokens);
		    break;

		case ':':
		    consume(new_token(TokenType::Colon, token), tokens);
		    break;

		case ';':
		    consume(new_token(TokenType::Semicolon, token), tokens);
		    break;

		case ',':
		    consume(new_token(TokenType::Comma, token), tokens);
		    break;

		case '+':
		    consume(new_token(TokenType::Plus, token), tokens);
		    break;

		case '-':
		    consume(new_token(TokenType::Minus, token), tokens);
		    break; 

		case '/':
		    consume(new_token(TokenType::Divides, token), tokens);
		    break;

		case '*':
		    consume(new_token(TokenType::Times, token), tokens);
		    break;

		case '%':
		    consume(new_token(TokenType::Mod, token), tokens);
		    break;

		default:
		    std::cerr << "Invalid token: \'" << token << "\'" << std::endl;
		    exit(1);
	    }
	}
    }

    return tokens;
}
