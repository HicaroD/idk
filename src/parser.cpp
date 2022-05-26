#include "idk/parser.h"

#include <iostream>
#include <unordered_set>
#include <vector>

#include "idk/lexer.h"

Parser::Parser(std::vector<Token> tokens_) {
  tokens = tokens_;
  cursor = tokens.begin();
}

Variable new_variable(TokenType type, std::string name, std::string value) {
  return Variable{{}, type, name, value};
}

Variable Parser::parse_variable_assignment() {
  TokenType variable_type = cursor->type;
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

  std::string value = cursor->id;
  cursor++;

  if (cursor->type != TokenType::Semicolon) {
    std::cerr << "Expected a ';' at the end of the statement" << std::endl;
    exit(1);
  }
  cursor++;
  return new_variable(variable_type, variable_name, value);
}

void Parser::generate_ast() {
  std::unordered_set<TokenType> data_types = {
      TokenType::Int, TokenType::Float, TokenType::Boolean, TokenType::String};

  while (cursor->type != TokenType::Eof) {
    if (data_types.contains(cursor->type)) {
      Variable variable = parse_variable_assignment();
      printf("VARIABLE '%s' => %s ;\n", variable.name.c_str(),
             variable.value.c_str());
    }
  }
}
