#ifndef PARSER_H
#define PARSER_H

#include <idk/lexer.h>

#include <string>
#include <vector>

enum class Type {
  Int,
  Float,
  Boolean,
};

struct ASTNode {};
struct Statement : ASTNode {};
struct Expression : ASTNode {};

struct Variable : ASTNode {
  TokenType type;
  std::string name;
  std::string value;
};

Variable new_variable(TokenType type, std::string name, std::string value);

class Parser {
 private:
  std::vector<Token> tokens;
  std::vector<Token>::iterator cursor;

 public:
  Parser(std::vector<Token> tokens_);

  Variable parse_variable_assignment();
  Expression parse_expression();

  void generate_ast();
};

#endif  // PARSER_H
