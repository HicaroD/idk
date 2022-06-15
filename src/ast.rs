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

// TODO: Implement function declaration

// All possible nodes for an AST
//
// 1. Assignment
//    int name = 12;
#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Assignment(Variable),
}
