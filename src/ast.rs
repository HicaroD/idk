use crate::lexer::Token;
use std::boxed::Box;

#[derive(PartialEq)]
pub enum Associativity {
    Left,
    Right,
    Undefined,
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    Bool,
    StringType,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Float(f64),
    Int(i32),
    StringLit(String),
    Char(char),
    Boolean(bool),
    BinaryExpr(Box<Expression>, Token, Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment(Variable),
}

#[derive(Debug, Clone)]
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
