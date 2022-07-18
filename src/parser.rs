use crate::{ast::*, backend::evaluate_ast, lexer::*};

use std::{collections::HashMap, str::FromStr};

pub struct Parser {
    tokens: Vec<Token>,
    current_token: Token,
    position: usize,
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
                Token::FloatNumber(value) | Token::IntNumber(value) => {
                    println!("ADD NUMBER TO STACK {:?}", value);
                    expressions.push(self.parse_number(value)?);
                }

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

    fn is_end_of_statement(&self) -> bool {
        self.current_token == Token::Semicolon
    }

    fn get_rpn_expression(&mut self, scope: &HashMap<String, Ast>) -> Result<Vec<Token>, String> {
        let mut operators: Vec<Token> = vec![];
        let mut operands: Vec<Token> = vec![];

        while !self.is_end_of_statement() {
            match &self.current_token {
                number if self.current_token.is_number() => {
                    operands.push(number.clone());
                }

                Token::Identifier(ident) => {
                    if let Some(Ast::Assignment(variable)) = scope.get(ident) {
                        println!("Found a variable: {:?}", variable);
                        let value = evaluate_ast(variable.value.clone())?;
                        let var = match &variable.var_type {
                            // TODO: Convert string to number on the lexer to avoid this "to_string"
                            Type::Int => Token::IntNumber(value.to_string()),
                            Type::Float => Token::FloatNumber(value.to_string()),
                            t => return Err(format!("Unsuported type: {:?}", t)),
                        };
                        operands.push(var);
                    } else {
                        return Err("Use of undeclared variable".to_string());
                    }
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

                        if top != Token::LeftPar && top.has_higher_precedence(op)
                            || top.has_same_precedence(op)
                                && op.get_associativity() == Associativity::Left
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

    fn parse_expression(&mut self, scope: &HashMap<String, Ast>) -> Result<Expression, String> {
        println!("PARSING EXPRESSION: {:?}", self.current_token);
        let rpn_expression = self.get_rpn_expression(scope)?;

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

    fn parse_assignment(&mut self, scope: &HashMap<String, Ast>) -> Result<Assignment, String> {
        let var_type = self.parse_type()?;
        self.advance();
        let name = self.parse_identifier()?;
        self.advance();
        self.parse_equal_sign()?;
        self.advance();
        let expression = self.parse_expression(scope)?;
        self.parse_semicolon()?;

        // let evaluated_expression = evaluate_ast(expression.clone())?;
        // println!("EVALUATED EXPRESSION: {}", evaluated_expression);

        let assignment = Assignment::new(var_type, name, expression);
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
        let mut symbol_table: HashMap<String, Ast> = HashMap::new();

        while self.current_token != Token::RightCurly {
            let statement = match &self.current_token {
                token if token.is_data_type_keyword() => {
                    let assignment = self.parse_assignment(&symbol_table)?;
                    symbol_table
                        .insert(assignment.name.clone(), Ast::Assignment(assignment.clone()));
                    Ast::Assignment(assignment)
                }
                _ => return Err(format!("Invalid token: {:?}", self.current_token)),
            };
            body.push(statement);
            self.advance();
        }
        Ok(Block::new(body, symbol_table))
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
            Block::new(vec![], HashMap::new()),
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

        let assignment = Ast::Assignment(Assignment::new(
            Type::Int,
            "a".to_string(),
            Expression::Int(12),
        ));

        let block = vec![assignment.clone()];
        let mut symbol_table = HashMap::new();
        symbol_table.insert("a".to_string(), assignment);

        let expected_function = Ast::Function(Function::new(
            "name".to_string(),
            vec![],
            Block::new(block, symbol_table),
            Some(Type::Int),
        ));
        assert_eq!(expected_function, *function_ast)
    }
}
