#include "idk/parser.h"

#include <iostream>
#include <unordered_set> 
#include <vector>
#include <stdlib.h>

#include "idk/lexer.h"

Parser::Parser(std::vector<Token> tokens_) {
  tokens = tokens_;
  cursor = tokens.begin();
}

std::string Parser::parse_identifier() {
  if (cursor->type != TokenType::Identifier) {
    std::cerr << "Expected an identifier" << std::endl;
    exit(1);
  }
  std::string variable_name = cursor->id;
  cursor++;
  return variable_name;
}

void Parser::parse_equal_sign() {
  if (cursor->type != TokenType::EqualSign) {
    std::cerr << "Expected an equal sign" << std::endl;
    exit(1);
  }
  cursor++;
}

void Parser::parse_semicolon() {
  if (cursor->type != TokenType::Semicolon) {
    std::cerr << "Expected a ';' at the end of the statement" << std::endl;
    exit(1);
  }
  cursor++;
}

Number::Number(TokenType t, double val) {
    type = t;
    value = val;
}

Expression Parser::parse_expression() { 
    if(cursor->type == TokenType::FloatNumber || cursor->type == TokenType::IntNumber) {
	double number_value = std::atof((cursor->id).c_str());
	return Number(cursor->type, number_value);
    } else {
	std::cerr << "Invalid expression" << std::endl;
	exit(1);
    }
}

Assignment::Assignment(TokenType t, std::string ident, Expression val) {
    type = t;
    identifier = ident;
    value = val;
}

Assignment Parser::parse_assignment() {
  TokenType variable_type = cursor->type;
  cursor++;

  std::string variable_name = parse_identifier();
  parse_equal_sign();

  Expression value = parse_expression();
  cursor++;

  parse_semicolon();
  return Assignment(variable_type, variable_name, value);
}

std::vector<ASTNode> Parser::generate_ast() {
  std::vector<ASTNode> ast;

  std::unordered_set<TokenType> data_types = {
      TokenType::Int, TokenType::Float, TokenType::Boolean, TokenType::String};

  while (cursor->type != TokenType::Eof) {
    if (data_types.contains(cursor->type)) {
      Assignment variable = parse_assignment();
      printf("Variable '%s'\n", variable.get_variable_name().c_str());
      ast.push_back(variable);
    }
  }

  return ast;
}
