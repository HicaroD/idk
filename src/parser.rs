use crate::{ast::*, lexer::*};

use std::{collections::HashMap, str::FromStr};

// TODO: The result of an expression is not always an f64
fn evaluate_ast(expression: Expression) -> Result<f64, String> {
    match expression {
        Expression::Int(value) => Ok(f64::from(value)),

        Expression::Float(value) => Ok(value),

        Expression::BinaryExpr(lhs, operation, rhs) => {
            let left = evaluate_ast(*lhs)?;
            let right = evaluate_ast(*rhs)?;

            match operation {
                Token::Plus => Ok(left + right),
                Token::Minus => Ok(left - right),
                Token::Times => Ok(left * right),
                Token::Divides => Ok(left / right),
                _ => Err(format!(
                    "Operator not implemented or invalid: {:?}",
                    operation
                )),
            }
        }

        _ => Err(format!("Expression not implemented: {:?}", expression)),
    }
}

#[derive(PartialEq)]
pub enum Associativity {
    Left,
    Right,
    Undefined,
}

pub struct Parser {
    tokens: Vec<Token>,
    current_token: Token,
    position: usize,
    // TODO: implement scope system (each variable has its own scope)
    // Each block has its own symbol table
    symbol_table: HashMap<String, Ast>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_token: Token::Eof,
            position: 0,
            symbol_table: HashMap::new(),
        }
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.current_token = self.tokens[self.position].clone();
            self.position += 1;
        }
    }

    fn from_rpn_to_ast(&self, rpn: Vec<Token>) -> Result<Expression, String> {
        let mut expressions: Vec<Expression> = vec![];

        for token in rpn.iter() {
            match token {
                Token::Number(value) => {
                    println!("ADD NUMBER TO STACK {:?}", value);
                    expressions.push(self.parse_number(value)?);
                }

                Token::Identifier(ident) => match self.symbol_table.get(ident) {
                    Some(ast) => match ast {
                        Ast::Assignment(assignment) => expressions.push(assignment.value.clone()),
                        _ => return Err(format!("Unexpected identifier: {:?}", ident)),
                    },
                    None => return Err(format!("Undefined variable or function: {:?}", ident)),
                },

                operator if operator.is_operator() => {
                    if expressions.len() >= 2 {
                        let rhs = Box::new(expressions.pop().unwrap());
                        let lhs = Box::new(expressions.pop().unwrap());
                        expressions.push(Expression::BinaryExpr(lhs, operator.clone(), rhs));
                    } else {
                        return Err("Error: Invalid expression".to_string());
                    }
                }

                _ => return Err("Invalid token on RPN expression".to_string()),
            }
        }

        if expressions.len() == 1 {
            Ok(expressions[0].clone())
        } else {
            Err("Error: Invalid RPN expression".to_string())
        }
    }

    fn parse_type(&self) -> Result<Type, String> {
        println!("PARSING TYPE: {:?}", self.current_token);
        self.current_token.as_type()
    }

    fn parse_identifier(&self) -> Result<String, String> {
        println!("PARSING IDENTIFIER: {:?}", self.current_token);
        if let Token::Identifier(ident) = &self.current_token {
            Ok(ident.to_string())
        } else {
            Err("Error while parsing identifier".to_string())
        }
    }

    fn parse_statement(&mut self) -> Result<Ast, String> {
        println!("PARSING STATEMENT: {:?}", self.current_token);
        if self.current_token.is_data_type_keyword() {
            return Ok(Ast::Assignment(self.parse_assignment()?));
        }
        Err("Invalid statement".to_string())
    }

    fn parse_number(&self, number: &str) -> Result<Expression, String> {
        if number.contains('.') {
            match f64::from_str(number) {
                Ok(value) => Ok(Expression::Float(value)),
                Err(err) => Err(format!("Couldn't parse float value: {:?}", err)),
            }
        } else {
            match i32::from_str(number) {
                Ok(value) => Ok(Expression::Int(value)),
                Err(err) => Err(format!("Couldn't parse integer value: {:?}", err)),
            }
        }
    }

    fn get_associativity(&self, operator: &Token) -> Associativity {
        match *operator {
            Token::Plus | Token::Minus | Token::Times | Token::Divides => Associativity::Left,
            _ => Associativity::Undefined,
        }
    }

    fn get_precedence(&self, token: &Token) -> i8 {
        match *token {
            Token::Plus => 1,
            Token::Minus => 1,
            Token::Times => 2,
            Token::Divides => 2,
            _ => -1,
        }
    }

    fn has_higher_precedence(&self, first_token: &Token, second_token: &Token) -> bool {
        self.get_precedence(first_token) > self.get_precedence(second_token)
    }

    fn has_same_precedence(&self, first_token: &Token, second_token: &Token) -> bool {
        self.get_precedence(first_token) == self.get_precedence(second_token)
    }

    fn is_end_of_statement(&self) -> bool {
        self.current_token == Token::Semicolon
    }

    fn get_rpn_expression(&mut self) -> Result<Vec<Token>, String> {
        let mut operators: Vec<Token> = vec![];
        let mut operands: Vec<Token> = vec![];

        loop {
            if self.is_end_of_statement() {
                break;
            }

            match &self.current_token {
                Token::Number(_) | Token::Identifier(_) => {
                    operands.push(self.current_token.clone());
                }

                Token::LeftPar => {
                    operators.push(self.current_token.clone());
                }

                Token::RightPar => {
                    let mut found_left_parenthesis = false;
                    while !operators.is_empty() {
                        if *operators.last().unwrap() == Token::LeftPar {
                            found_left_parenthesis = true;
                            break;
                        } else {
                            let op = operators.pop().unwrap();
                            operands.push(op);
                        }
                    }

                    if operators.is_empty() && !found_left_parenthesis {
                        return Err("Error: Left parenthesis not found".to_string());
                    } else {
                        // DISCARD LEFT PARENTHESIS AT THE TOP
                        operators.pop().unwrap();
                    }
                }

                // TODO: Refactor excessive clone
                op if op.is_operator() => {
                    while !operators.is_empty() {
                        let top = operators.last().unwrap().clone();

                        if top != Token::LeftPar && self.has_higher_precedence(&top, &op)
                            || self.has_same_precedence(&top, &op)
                                && self.get_associativity(&op) == Associativity::Left
                        {
                            operands.push(operators.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                    operators.push(op.clone());
                }

                _ => {
                    return Err(format!("Error: Invalid token: {:?}", self.current_token));
                }
            };
            self.advance();
        }

        while !operators.is_empty() {
            let top = operators.last().unwrap().clone();
            if top == Token::LeftPar {
                return Err("Error: Mismatched parenthesis".to_string());
            }
            operands.push(operators.pop().unwrap());
        }

        Ok(operands)
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        println!("PARSING EXPRESSION: {:?}", self.current_token);
        let rpn_expression = self.get_rpn_expression()?;

        for rpn_token in rpn_expression.iter() {
            println!("RPN: {:?}", rpn_token);
        }
        let ast = self.from_rpn_to_ast(rpn_expression)?;
        println!("AST: {:?}", ast);
        Ok(ast)
    }

    fn parse_semicolon(&self) -> Result<(), String> {
        println!("PARSING SEMICOLON: {:?}", self.current_token);
        if self.current_token != Token::Semicolon {
            Err(format!(
                "Invalid token {:?}. Expected semicolon at the end of statement",
                self.current_token
            ))
        } else {
            Ok(())
        }
    }

    fn parse_equal_sign(&self) -> Result<(), String> {
        println!("PARSING EQUAL SIGN: {:?}", self.current_token);
        if self.current_token != Token::EqualSign {
            Err(format!(
                "Invalid token {:?}. Expected an equal sign",
                self.current_token
            ))
        } else {
            Ok(())
        }
    }

    fn parse_assignment(&mut self) -> Result<Assignment, String> {
        let var_type = self.parse_type()?;
        self.advance();
        let name = self.parse_identifier()?;
        self.advance();
        self.parse_equal_sign()?;
        self.advance();
        let expression = self.parse_expression()?;
        self.parse_semicolon()?;

        let evaluated_expression = evaluate_ast(expression.clone())?;
        println!("EVALUATED EXPRESSION: {}", evaluated_expression);

        let assignment = Assignment::new(var_type, name.clone(), expression);
        self.symbol_table
            .insert(name, Ast::Assignment(assignment.clone()));
        Ok(assignment)
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Parameter>, String> {
        if self.current_token != Token::LeftPar {
            return Err(format!(
                "Unexpected token on function parameter parsing: {:?}",
                &self.current_token
            ));
        }
        self.advance();

        let mut parameters: Vec<Parameter> = vec![];

        while self.current_token != Token::RightPar {
            let parameter_type = self.parse_type()?;
            self.advance();
            let parameter_name = self.parse_identifier()?;
            parameters.push(Parameter::new(parameter_type, parameter_name));
            self.advance();

            if self.current_token == Token::Comma {
                self.advance();
            }
        }
        Ok(parameters)
    }

    // TODO: That function can have two types of errors (return None):
    //       Either a function declaration doesn't specify the return type or
    //       it is not well formed
    //
    //       I'll need to know when these different errors happen.
    fn parse_function_return_type(&mut self) -> Result<Type, String> {
        self.advance();

        if self.current_token == Token::LeftCurly {
            return Ok(Type::Void);
        } else if self.current_token == Token::Colon {
            self.advance();

            // Function is not well formed
            if !self.current_token.is_data_type_keyword() {
                return Err("Expected return type for function".to_string());
            }

            let variable_type = self.parse_type()?;
            self.advance();
            return Ok(variable_type);
        }

        Err("Expected left curly brace or return type declaration".to_string())
    }

    fn parse_block(&mut self) -> Result<Block, String> {
        if self.current_token != Token::LeftCurly {
            return Err("Expected a left curly brace".to_string());
        }
        self.advance();

        let mut body: Vec<Ast> = vec![];

        while self.current_token != Token::RightCurly {
            let statement = match &self.current_token {
                token if token.is_data_type_keyword() => Ast::Assignment(self.parse_assignment()?),
                _ => return Err(format!("Invalid token: {:?}", self.current_token)),
            };
            body.push(statement);
            self.advance();
        }
        Ok(Block::new(body))
    }

    fn parse_function(&mut self) -> Result<Function, String> {
        println!("PARSING FUNCTION: {:?}", self.current_token);
        self.advance();

        let function_name = self.parse_identifier()?;
        self.advance();
        let parameters = self.parse_function_parameters()?;
        let return_type = match self.parse_function_return_type()? {
            Type::Void => None,
            t => Some(t),
        };
        let body: Block = self.parse_block()?;

        if self.current_token != Token::RightCurly {
            return Err(format!(
                "Unexpected token on function parsing: {:?}",
                self.current_token
            ));
        }
        self.advance();

        let function = Function::new(function_name.clone(), parameters, body, return_type);
        self.symbol_table
            .insert(function_name, Ast::Function(function.clone()));
        Ok(function)
    }

    pub fn generate_ast(&mut self) -> Result<Vec<Ast>, String> {
        self.advance();
        let mut ast: Vec<Ast> = vec![];

        while self.current_token != Token::Eof {
            match &self.current_token {
                _ if self.current_token.is_data_type_keyword() => {
                    let variable_declaration = self.parse_statement()?;
                    ast.push(variable_declaration.clone());
                    println!("CURRENT STATEMENT: {:?}", variable_declaration);
                    self.advance();
                }

                Token::KeywordFn => {
                    let function = self.parse_function()?;
                    println!("FUNCTION: {:?}", function);
                    ast.push(Ast::Function(function));
                    self.advance();
                }

                _ => {
                    return Err(format!(
                        "Error: Invalid token on AST parsing: {:?}",
                        self.current_token
                    ))
                }
            }
        }

        Ok(ast)
    }
}

// TODO: Refactor test (too verbose, maybe?)
//       Test symbol table and function body (block)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_declaration() {
        let input = "float variable_name = 8;\n".chars().collect::<Vec<char>>();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let variable_ast = parser.generate_ast().unwrap();

        let value = Expression::Int(8);
        let variable = Assignment::new(Type::Float, "variable_name".to_string(), value);
        let expected_variable_ast = Ast::Assignment(variable);

        assert_eq!(variable_ast[0], expected_variable_ast);
    }

    #[test]
    fn test_ast_evaluation() {
        let input = "float variable_name = 8 / 4 / 2;\n"
            .chars()
            .collect::<Vec<char>>();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let variable_ast = parser.generate_ast().unwrap();

        let var = &variable_ast[0];
        if let Ast::Assignment(variable) = var {
            let expression = variable.value.clone();
            let value = evaluate_ast(expression).unwrap();
            assert_eq!(value, 1.0);
        } else {
            panic!("This should be a variable declaration!");
        }
    }

    #[test]
    fn test_function_declaration_with_empty_body() {
        let input = "fn name(): int {}\n".chars().collect::<Vec<char>>();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let function_ast = &parser.generate_ast().unwrap()[0];

        let function = Function::new(
            "name".to_string(),
            vec![],
            Block::new(vec![]),
            Some(Type::Int),
        );
        let expected_result = Ast::Function(function);
        assert_eq!(expected_result, *function_ast)
    }

    #[test]
    fn test_function_declaration() {
        let input = "fn name(): int {int a = 12;}\n"
            .chars()
            .collect::<Vec<char>>();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let function_ast = &parser.generate_ast().unwrap()[0];

        let block = vec![Ast::Assignment(Assignment::new(
            Type::Int,
            "a".to_string(),
            Expression::Int(12),
        ))];
        let expected_function = Ast::Function(Function::new(
            "name".to_string(),
            vec![],
            Block::new(block),
            Some(Type::Int),
        ));
        assert_eq!(expected_function, *function_ast)
    }
}
