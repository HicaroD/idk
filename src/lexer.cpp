#include <idk/lexer.h>

#include <iostream>
#include <cstdlib>
#include <cctype>
#include <vector>

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
    if(source_code.empty()) {
	exit(1);
    } else if(!is_eof()) {
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
	std::cout << "Current char: " << current_char << std::endl;
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

	// TODO: Refactoring
	//       - This conversion of token to string is terrible
	//       - Could I simplify these if statements?

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

	} else if(current_char == '(' || current_char == ')') {
	    consume(new_token(TokenType::Parenthesis, token), tokens);
	}

	else if(current_char == '{' || current_char == '}') {
	    consume(new_token(TokenType::CurlyBraces, token), tokens);

	} else if(current_char == '[' || current_char == ']') {
	    consume(new_token(TokenType::Parenthesis, token), tokens);

	} else if(current_char == '=') {
	    consume(new_token(TokenType::EqualSign, token), tokens);

	} else if(current_char == ':') {
	    consume(new_token(TokenType::Colon, token), tokens);

	} else if(current_char == ';') {
	    consume(new_token(TokenType::Semicolon, token), tokens);

	} else if(current_char == ',') {
	    consume(new_token(TokenType::Comma, token), tokens);

	} else if(current_char == '+') {
	    consume(new_token(TokenType::Plus, token), tokens);

	} else if(current_char == '-') {
	    consume(new_token(TokenType::Minus, token), tokens);

	} else {
	    std::cerr << "Invalid token: \'" << token << "\'" << std::endl;
	    exit(1);
	}
    }

    return tokens;
}
