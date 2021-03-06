use crate::lexer::Token;
use std::boxed::Box;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Void,
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
    // FIXME: Performance issues with Box
    BinaryExpr(Box<Expression>, Token, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub var_type: Type,
    pub name: String,
    pub value: Expression,
}

impl Assignment {
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
    pub parameter_type: Type,
    pub name: String,
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
pub struct Block {
    pub itens: Vec<Ast>,
    pub symbol_table: HashMap<String, Ast>,
}

impl Block {
    pub fn new(itens: Vec<Ast>, symbol_table: HashMap<String, Ast>) -> Self {
        Self {
            itens,
            symbol_table,
        }
    }
}

// TODO: Implement return instruction on function body
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Block,
    pub return_type: Option<Type>,
}

impl Function {
    pub fn new(
        name: String,
        parameters: Vec<Parameter>,
        body: Block,
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
    Assignment(Assignment),
    Function(Function),
}
