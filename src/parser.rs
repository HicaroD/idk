use crate::{ast::*, lexer::*};

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

fn is_operator(token: &Token) -> bool {
    let operators: HashSet<Token> = HashSet::from([
        Token::Plus,
        Token::Minus,
        Token::Mod,
        Token::Divides,
        Token::Times,
    ]);
    operators.get(token).is_some()
}

fn is_data_type_keyword(token: &Token) -> bool {
    let data_types: HashSet<KeywordId> = HashSet::from([
        KeywordId::Int,
        KeywordId::Float,
        KeywordId::Bool,
        KeywordId::StringKeyword,
    ]);

    if let Token::Keyword(keyword) = token {
        data_types.get(keyword).is_some()
    } else {
        false
    }
}

#[derive(PartialEq)]
pub enum Associativity {
    Left,
    Right,
    Undefined,
}

struct ASTEvaluator {}

impl ASTEvaluator {
    // TODO: The result of an expression is not always an f64
    fn evaluate(expression: Expression) -> Result<f64, String> {
        match expression {
            Expression::Int(value) => Ok(f64::from(value)),

            Expression::Float(value) => Ok(value),

            Expression::BinaryExpr(lhs, operation, rhs) => {
                let left = ASTEvaluator::evaluate(*lhs)?;
                let right = ASTEvaluator::evaluate(*rhs)?;

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
}

struct SymbolTable {
    items: HashMap<String, Ast>,
}

impl SymbolTable {
    fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current_token: Token,
    position: usize,
    symbol_table: SymbolTable,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_token: Token::EOF,
            position: 0,
            symbol_table: SymbolTable::new(),
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

                operator if is_operator(&operator) => {
                    if expressions.len() >= 2 {
                        let rhs = Box::new(expressions.pop().unwrap());
                        let lhs = Box::new(expressions.pop().unwrap());
                        expressions.push(Expression::BinaryExpr(lhs, operator.clone(), rhs));
                    } else {
                        return Err(format!("Error: Invalid expression"));
                    }
                }

                _ => return Err(format!("Invalid token on RPN expression")),
            }
        }

        if expressions.len() == 1 {
            Ok(expressions[0].clone())
        } else {
            Err(format!("Error: Invalid RPN expression"))
        }
    }

    fn parse_type(&self) -> Result<Type, String> {
        println!("PARSING TYPE: {:?}", self.current_token);

        if let Token::Keyword(keyword) = self.current_token {
            keyword.is_type()
        } else {
            Err(format!("Unexpected token on variable declaration"))
        }
    }

    fn parse_identifier(&self) -> Result<String, String> {
        println!("PARSING IDENTIFIER: {:?}", self.current_token);
        if let Token::Identifier(ident) = &self.current_token {
            Ok(ident.to_string())
        } else {
            Err(format!("Error while parsing identifier"))
        }
    }

    fn parse_statement(&mut self) -> Result<Ast, String> {
        println!("PARSING STATEMENT: {:?}", self.current_token);
        if is_data_type_keyword(&self.current_token) {
            return Ok(Ast::Assignment(self.parse_assignment()?));
        }
        return Err(format!("Invalid statement"));
    }

    fn parse_number(&self, number: &str) -> Result<Expression, String> {
        if number.contains('.') {
            match f64::from_str(&number) {
                Ok(value) => Ok(Expression::Float(value)),
                Err(err) => Err(format!("Couldn't parse float value: {:?}", err)),
            }
        } else {
            match i32::from_str(&number) {
                Ok(value) => Ok(Expression::Int(value)),
                Err(err) => Err(format!("Couldn't parse integer value: {:?}", err)),
            }
        }
    }

    fn get_associativity(&self, operator: Token) -> Associativity {
        match operator {
            Token::Plus | Token::Minus | Token::Times | Token::Divides => Associativity::Left,
            _ => Associativity::Undefined,
        }
    }

    fn get_precedence(&self, token: Token) -> i8 {
        match token {
            Token::Plus => 1,
            Token::Minus => 1,
            Token::Times => 2,
            Token::Divides => 2,
            _ => -1,
        }
    }

    fn has_higher_precedence(&self, first_token: Token, second_token: Token) -> bool {
        self.get_precedence(first_token) > self.get_precedence(second_token)
    }

    fn has_same_precedence(&self, first_token: Token, second_token: Token) -> bool {
        self.get_precedence(first_token) == self.get_precedence(second_token)
    }

    fn is_end_of_statement(&self) -> bool {
        self.current_token == Token::Semicolon
    }

    fn get_rpn_expression(&mut self) -> Result<Vec<Token>, String> {
        let mut operators: Vec<Token> = vec![];
        let mut operands: Vec<Token> = vec![];

        loop {
            println!("CURRENT TOKEN: {:?}", self.current_token);
            if self.is_end_of_statement() {
                break;
            }

            match &self.current_token {
                Token::Number(_) => {
                    println!("ADDING NUMBER TO OPERANDS: {:?}", self.current_token);
                    operands.push(self.current_token.clone());
                }

                Token::LeftPar => {
                    println!(
                        "ADDING LEFT PARENTHESIS TO OPERATORS: {:?}",
                        self.current_token
                    );
                    operators.push(self.current_token.clone());
                }

                Token::RightPar => {
                    println!("FOUND RIGHT PARENTHESIS");

                    let mut found_left_parenthesis = false;
                    while !operators.is_empty() {
                        if *operators.last().unwrap() == Token::LeftPar {
                            println!("FOUND LEFT PARENTHESIS");
                            found_left_parenthesis = true;
                            break;
                        } else {
                            let op = operators.pop().unwrap();
                            operands.push(op);
                        }
                    }

                    if operators.is_empty() && !found_left_parenthesis {
                        return Err(format!("Error: Left parenthesis not found"));
                    } else {
                        // DISCARD LEFT PARENTHESIS AT THE TOP
                        println!("FOUND LEFT PARENTHESIS -> DISCARDING NOW");
                        operators.pop().unwrap();
                    }
                }

                // TODO: Refactor excessive clone
                op if is_operator(&op) => {
                    println!("FOUND OPERATOR: {:?}", op);
                    while !operators.is_empty() {
                        let top = operators.last().unwrap().clone();
                        println!("TOP OPERATOR ON OPERATOR MATCHING: {:?}", top);
                        println!("CURRENT OPERATOR ON OPERATOR MATCHING: {:?}", op);
                        if top == Token::LeftPar {
                            break;
                        }
                        if self.has_higher_precedence(top.clone(), op.clone())
                            || self.has_same_precedence(top.clone(), op.clone())
                                && self.get_associativity(op.clone()) == Associativity::Left
                        {
                            println!("IS {:?} LOWER THAN {:?}", op, top);
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

            println!("ADVANCING TOKEN: {:?}", self.current_token);
            self.advance();
        }

        while !operators.is_empty() {
            let top = operators.last().unwrap().clone();
            if top == Token::LeftPar {
                return Err(format!("Error: Mismatched parenthesis"));
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

        let evaluated_expression = ASTEvaluator::evaluate(expression.clone())?;
        println!("EVALUATED EXPRESSION: {}", evaluated_expression);
        Ok(Assignment::new(var_type, name, expression))
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

    // TODO: That function can two types of errors:
    //       Either a function declaration doesn't specify the return type or
    //       it is not well formed
    //
    //       I'll need to know when these errors happen.
    fn parse_function_return_type(&mut self) -> Option<Type> {
        // Function with no return type
        self.advance();

        if self.current_token == Token::LeftCurly {
            return None;
        }

        if self.current_token == Token::Colon {
            self.advance();

            // Function is not well formed
            if !is_data_type_keyword(&self.current_token) {
                return None;
            }
            // TODO: Avoid "unwrap"
            let variable_type = self.parse_type().unwrap();
            self.advance();
            return Some(variable_type);
        }

        // Function is not well formed
        return None;
    }

    // TODO: Parse function body (list of statements)
    // TODO: Create symbol table
    fn parse_function_body(&mut self) -> Result<Block, String> {
        if self.current_token != Token::LeftCurly {
            return Err(format!("Expected a left curly brace"));
        }
        self.advance();

        let mut body: Vec<Ast> = vec![];

        while self.current_token != Token::RightCurly {
            let statement = match &self.current_token {
                data_type if is_data_type_keyword(&data_type) => {
                    Ast::Assignment(self.parse_assignment()?)
                }
                _ => return Err(format!("Invalid token: {:?}", self.current_token)),
            };
            self.advance();
            body.push(statement);
        }
        Ok(Block::new(body))
    }

    fn parse_function(&mut self) -> Result<Function, String> {
        println!("PARSING FUNCTION: {:?}", self.current_token);
        self.advance();

        let function_name = self.parse_identifier()?;
        self.advance();

        let parameters = self.parse_function_parameters()?;

        // Assuming that function is always well formed and "None" means that the function has
        // no return type
        let return_type = self.parse_function_return_type();

        let body: Block = self.parse_function_body()?;

        if self.current_token != Token::RightCurly {
            return Err(format!(
                "Unexpected token on function parsing: {:?}",
                self.current_token
            ));
        }
        self.advance();
        Ok(Function::new(function_name, parameters, body, return_type))
    }

    pub fn generate_ast(&mut self) -> Result<Vec<Ast>, String> {
        self.advance();
        let mut ast: Vec<Ast> = vec![];

        while self.current_token != Token::EOF {
            match &self.current_token {
                _ if is_data_type_keyword(&self.current_token) => {
                    let variable_declaration = self.parse_statement()?;
                    ast.push(variable_declaration.clone());
                    println!("CURRENT STATEMENT: {:?}", variable_declaration);
                    self.advance();
                }

                Token::Keyword(keyword) => match keyword {
                    KeywordId::Fn => {
                        let function = self.parse_function()?;
                        println!("FUNCTION: {:?}", function);
                        ast.push(Ast::Function(function));
                        self.advance();
                    }

                    _ => return Err(format!("Unexpected keyword: {:?}", keyword)),
                },

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
            let value = ASTEvaluator::evaluate(expression).unwrap();
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
}
