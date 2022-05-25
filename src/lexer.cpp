#include "idk/lexer.h"

#include <cctype>
#include <cstdlib>
#include <iostream>
#include <unordered_map>
#include <vector>

Lexer::Lexer(const std::vector<char> &source) {
  source_code = source;
  current_char = '0';
  position = 0;
}

Token new_token(TokenType type, std::string id) { return Token{type, id}; }

bool equals(std::string identifier, std::string keyword) {
  return identifier.compare(keyword) == 0;
}

Token classify_identifier(std::string identifier) {
  std::unordered_map<std::string, TokenType> keywords = {
      {"def", TokenType::Def},     {"return", TokenType::Return},
      {"if", TokenType::If},       {"elif", TokenType::Elif},
      {"else", TokenType::Else},   {"int", TokenType::Int},
      {"float", TokenType::Float}, {"bool", TokenType::Boolean},
  };

  if (keywords.contains(identifier)) {
    return new_token(keywords[identifier], identifier);
  }

  return new_token(TokenType::Identifier, identifier);
}

void Lexer::skip_whitespace() {
  while (isspace(current_char)) {
    advance();
  }
}

void Lexer::advance() {
  if (!is_eof()) {
    current_char = source_code[position++];
  }
}

bool Lexer::is_eof() { return position == int(source_code.size()); }

Number Lexer::get_number() {
  std::string number;
  number += current_char;
  advance();

  bool is_float = false;

  while (isdigit(current_char) || current_char == '.') {
    if (current_char == '.') {
      is_float = true;
    }
    number += current_char;
    advance();
  }

  return is_float ? Number{{TokenType::Number, number}, NumberKind::Float}
                  : Number{{TokenType::Number, number}, NumberKind::Int};
}

std::string Lexer::get_identifier() {
  std::string identifier;
  identifier += current_char;

  advance();
  while ((isalnum(current_char) || current_char == '_')) {
    identifier += current_char;
    advance();
  }
  return identifier;
}

void Lexer::consume(Token token, std::vector<Token> &tokens) {
  tokens.push_back(token);
  advance();
}

std::vector<Token> Lexer::tokenize() {
  std::vector<Token> tokens;

  while (!is_eof()) {
    skip_whitespace();

    std::string token;
    token = current_char;

    if (isalpha(current_char)) {
      std::string identifier = get_identifier();
      Token token = classify_identifier(identifier);
      tokens.push_back(token);
    } else if (isdigit(current_char)) {
      Number number = get_number();
      tokens.push_back(number);
    } else {
      switch (current_char) {
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

  tokens.push_back(new_token(TokenType::Eof, "EOF"));
  return tokens;
}
