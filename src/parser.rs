use crate::ast::*;
use crate::lexer::*;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current_token: Token,
    pub position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_token: Token::EOF,
            position: 0,
        }
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.current_token = self.tokens[self.position].clone();
            self.position += 1;
        }
    }

    // TODO: Refactor this function (it will get worse when I extend the language with more types)
    fn is_data_type(&self) -> bool {
        match self.current_token {
            Token::Keyword(KeywordId::Int)
            | Token::Keyword(KeywordId::Float)
            | Token::Keyword(KeywordId::Bool) => true,

            _ => false,
        }
    }

    fn parse_type(&self) -> Result<Type, String> {
        println!("PARSING TYPE: {:?}", self.current_token);
        if let Token::Keyword(keyword_id) = self.current_token {
            match keyword_id {
                KeywordId::Int => Ok(Type::Int),
                KeywordId::Float => Ok(Type::Float),
                KeywordId::Bool => Ok(Type::Bool),
                _ => Err("Error while parsing variable type".to_string()),
            }
        } else {
            Err("Unexpected token on variable declaration".to_string())
        }
    }

    fn parse_identifier(&self) -> Result<String, String> {
        println!("PARSING IDENTIFIER: {:?}", self.current_token);
        if let Token::Identifier(ident) = &self.current_token {
            Ok(ident.to_string())
        } else {
            Err("Error while parsing identifier".to_string())
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        println!("PARSING STATEMENT: {:?}", self.current_token);
        if self.is_data_type() {
            return Ok(Statement::Assignment(self.parse_assignment()?));
        }
        return Err("Invalid statement".to_string());
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        println!("PARSING EXPRESSION: {:?}", self.current_token);
        match &self.current_token {
            Token::Number(number) => {
                // FIXME: Bad assumption that this code will never failure
                let value = f64::from_str(&number).unwrap();
                Ok(Expression::Number(value))
            }
            _ => Err("Invalid expression".to_string()),
        }
    }

    fn parse_semicolon(&self) -> Result<(), String> {
        println!("PARSING SEMICOLON: {:?}", self.current_token);
        if self.current_token != Token::SpecialChar(SpecialCharId::Semicolon) {
            Err("Expected semicolon at the end of statement".to_string())
        } else {
            Ok(())
        }
    }

    fn parse_equal_sign(&self) -> Result<(), String> {
        println!("PARSING EQUAL SIGN: {:?}", self.current_token);
        if self.current_token != Token::SpecialChar(SpecialCharId::EqualSign) {
            Err("Expected equal sign".to_string())
        } else {
            Ok(())
        }
    }

    fn parse_assignment(&mut self) -> Result<Variable, String> {
        let var_type = self.parse_type()?;
        self.advance();
        let name = self.parse_identifier()?;
        self.advance();
        self.parse_equal_sign()?;
        self.advance();
        let expression = self.parse_expression()?;
        self.advance();
        self.parse_semicolon()?;
        Ok(Variable::new(var_type, name, expression))
    }

    pub fn generate_ast(&mut self) -> Result<(), String> {
        self.advance();

        while self.current_token != Token::EOF {
            let statement = self.parse_statement()?;
            println!("CURRENT STATEMENT: {:?}", statement);
            self.advance();
        }

        Ok(())
    }
}
