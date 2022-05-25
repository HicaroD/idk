#ifndef LEXER_H
#define LEXER_H

#include <string>
#include <vector>

enum class TokenType
{
    // Keywords
    Def,
    Return,
    If,
    Elif,
    Else,

    Identifier,
    Number,

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

    // Type
    Int,
    Float,
    Boolean,

    Eof,
};

struct Token
{
    TokenType type;
    std::string id;
};

enum class NumberKind
{
    Int,
    Float,
    Boolean,
};

struct Number : Token
{
    NumberKind kind;
};

Token new_token(TokenType type, std::string id);
Token classify_identifier(std::string identifier);

class Lexer
{
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
    std::vector<Token> tokenize();
    std::string get_identifier();
    Number get_number();
};

#endif // LEXER_H
