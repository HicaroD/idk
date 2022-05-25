#include "idk/parser.h"

#include <iostream>
#include <vector>

#include "idk/lexer.h"

Parser::Parser(std::vector<Token> tokens_) {
  tokens = tokens_;
  cursor = tokens.begin();
}

Variable new_variable(Type type, std::string name, Expression value) {
  return Variable{{}, type, name, value};
}

NumberExpr new_number_expression(NumberKind kind, std::string value) {
  return NumberExpr{{}, kind, value};
}

Expression Parser::parse_expression() {
  if (cursor->type == TokenType::Number) {
    return new_number_expression(cursor->kind, cursor->id);
  } else {
    std::cerr << "Invalid expression" << std::endl;
    exit(1);
  }
}

Variable Parser::parse_variable_assignment() {
  cursor++;
  if (cursor->type != TokenType::Identifier) {
    std::cerr << "Expected an identifier" << std::endl;
    exit(1);
  }
  std::string variable_name = cursor->id;
  cursor++;

  if (cursor->type != TokenType::EqualSign) {
    std::cerr << "Expected an equal sign" << std::endl;
    exit(1);
  }
  cursor++;

  Expression value = parse_expression();
  cursor++;

  if (cursor->type != TokenType::Semicolon) {
    std::cerr << "Expected a ';' at the end of the statement" << std::endl;
    exit(1);
  }

  return new_variable(Type::Float, variable_name, value);
}

void Parser::generate_ast() {
  if (cursor->type == TokenType::Float) {
    Variable variable = parse_variable_assignment();
    printf("float %s = %s;\n", variable.name.c_str(), variable.value.c_str());
  }
}
