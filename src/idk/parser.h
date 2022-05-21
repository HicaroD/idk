#ifndef PARSER_H
#define PARSER_H

#include <idk/lexer.h>

#include <vector>

enum class Type {
    Int,
    Float,
    Boolean,
};

struct Parameter {
    std::string name;
    Type type;
};

struct ASTNode {};

struct Variable : ASTNode {
    std::string name;
    Type type;

    // It is not quite a string, it will be changed in the future
    std::string value;
};

struct Function : ASTNode {
    std::string return_type;
    std::string name;
    std::vector<Parameter> parameters;
    // TODO: Declare the body of the function
};

// TODO: Declare parser functions (one for each non-terminal)
class Parser {};

#endif // PARSER_H 
