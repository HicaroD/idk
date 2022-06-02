#ifndef PARSER_H
#define PARSER_H

#include <idk/lexer.h>

#include <string>
#include <vector>

class ASTNode {};

class Expression : public ASTNode {};
class Statement : public ASTNode {};

class Number : public Expression {
    private:
	TokenType type;
	double value;
    public:
	Number(TokenType type, double value);
};

class Assignment : public Statement {
    private:
	TokenType type;
	std::string identifier;
	Expression value;
    public:
	Assignment(TokenType type, std::string identifier, Expression value);
	std::string get_variable_name() { return identifier; }
};


class Parser {
 private:
  std::vector<Token> tokens;
  std::vector<Token>::iterator cursor;

 public:
  Parser(std::vector<Token> tokens_);

  Assignment parse_assignment();
  Expression parse_expression();

  std::string parse_identifier();
  void parse_semicolon();
  void parse_equal_sign();

  std::vector<ASTNode> generate_ast();
};

#endif  // PARSER_H
