#include "idk/parser.h"
#include "idk/lexer.h"

#include <iostream>
#include <vector>

Parser::Parser(std::vector<Token> tokens_) {
    tokens = tokens_;
    cursor = tokens.begin();
}

Variable new_variable(Type type, std::string name, std::string value) {
    return Variable { {}, type, name, value };
}

Variable Parser::parse_variable_assignment(std::vector<Token>::iterator* current_token) {
    current_token++;
    if(current_token->type != TokenType::Identifier) {
	std::cerr << "Expected an identifier" << std::endl;
	exit(1);
    }
    std::string variable_name = current_token->id;
    current_token++;

    if(current_token->type != TokenType::EqualSign) {
	std::cerr << "Expected an equal sign" << std::endl;
	exit(1);
    }
    current_token++;

    // TODO: It must be a float, not any kind of number
    if(current_token->type != TokenType::Number) {
	std::cerr << "Expected a number" << std::endl;
	exit(1);
    }
    std::string value = current_token->id;
    current_token++;

    if(current_token->type != TokenType::Semicolon) {
	std::cerr << "Expected a ';' at the end of the statement" << std::endl;
	exit(1);
    }

    return new_variable(TokenType::Float, name, value);
}

void Parser::generate_ast() {
    for(auto current_token = cursor; current_token != tokens.end(); current_token++) {
	if(current_token->type == TokenType::Float) {
	    Variable variable = parse_variable_assignment(&current_token);
	    printf("float %s = %s", variable.name.c_str(), variable.value.c_str());
	    break;
	}
    }
}
