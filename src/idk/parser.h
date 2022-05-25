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

struct Variable : Statement {
  Type type;
  std::string name;
  // It is not quite a string, it can be anything
  std::string value;
};

Variable new_variable(Type type, std::string name, std::string value);

class Parser {
 private:
  std::vector<Token> tokens;
  std::vector<Token>::iterator cursor;

 public:
  Parser(std::vector<Token> tokens_);

  Variable parse_variable_assignment();

  void generate_ast();
};

#endif  // PARSER_H
