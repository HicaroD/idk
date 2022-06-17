use crate::lexer::Token;
use std::boxed::Box;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    StringType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Float(f64),
    Int(i32),
    StringLit(String),
    Char(char),
    Boolean(bool),
    BinaryExpr(Box<Expression>, Token, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub var_type: Type,
    pub name: String,
    pub value: Expression,
}

impl Variable {
    pub fn new(var_type: Type, name: String, value: Expression) -> Self {
        Self {
            var_type,
            name,
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    parameter_type: Type,
    name: String,
    // TODO: add optional default value for a parameter
}

impl Parameter {
    pub fn new(parameter_type: Type, name: String) -> Self {
        Self {
            parameter_type,
            name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition {
    name: String,
    parameters: Vec<Parameter>,
    body: Vec<Ast>,
    return_type: Option<Type>,
}

impl FunctionDefinition {
    pub fn new(
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<Ast>,
        return_type: Option<Type>,
    ) -> Self {
        Self {
            name,
            parameters,
            body,
            return_type,
        }
    }
}

// All possible nodes for an AST
//
// 1. Assignment
//    int name = 12;
//
// 2. Function
//    fn sum(int a, int b): int {
//        return a + b;
//    }
#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Assignment(Variable),
    Function(FunctionDefinition),
}
