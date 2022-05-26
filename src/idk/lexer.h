#ifndef LEXER_H
#define LEXER_H

#include <string>
#include <vector>

enum class TokenType {
  // Keywords
  Def,
  Return,
  If,
  Elif,
  Else,
  Int,
  Float,
  Boolean,
  String,

  Identifier,

  // Data types
  FloatNumber,
  IntNumber,
  True,
  False,
  Str,

  // Special characters
  Parenthesis,
  CurlyBraces,
  Brackets,
  EqualSign,
  Semicolon,
  Colon,
  Comma,

  // Operators
  Plus,
  Minus,
  Mod,
  Divides,
  Times,

  Eof,
};

struct Token {
  TokenType type;
  std::string id;
};

Token new_token(TokenType type, std::string id);
Token classify_identifier(std::string identifier);

class Lexer {
 private:
  std::vector<char> source_code;
  char current_char;
  int position;

 public:
  Lexer(const std::vector<char> &source);

  void advance();
  bool is_eof();
  void skip_whitespace();
  void consume(Token token, std::vector<Token> &tokens);

  std::string get_identifier();
  Token get_number();
  Token get_string();

  std::vector<Token> tokenize();
};

#endif  // LEXER_H
