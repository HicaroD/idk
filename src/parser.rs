use crate::ast::*;
use crate::lexer::*;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Parser {
    tokens: Vec<Token>,
    current_token: Token,
    position: usize,
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

    // TODO: Refactor this function (HashSet could help)
    fn is_operator(&self) -> bool {
        let operators: HashSet<Token> = HashSet::from([
            Token::Plus,
            Token::Minus,
            Token::Mod,
            Token::Divides,
            Token::Times,
        ]);
        operators.get(&self.current_token).is_some()
    }

    fn is_data_type(&self) -> bool {
        let data_types: HashSet<KeywordId> = HashSet::from([
            KeywordId::Int,
            KeywordId::Float,
            KeywordId::Bool,
            KeywordId::StringKeyword,
        ]);

        if let Token::Keyword(keyword) = self.current_token {
            data_types.get(&keyword).is_some()
        } else {
            false
        }
    }

    fn parse_type(&self) -> Result<Type, String> {
        println!("PARSING TYPE: {:?}", self.current_token);

        if let Token::Keyword(keyword) = self.current_token {
            match keyword {
                KeywordId::Int => Ok(Type::Int),
                KeywordId::Float => Ok(Type::Float),
                KeywordId::Bool => Ok(Type::Bool),
                KeywordId::StringKeyword => Ok(Type::StringType),
                _ => Err(format!("Error while parsing variable type")),
            }
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

    fn parse_statement(&mut self) -> Result<Statement, String> {
        println!("PARSING STATEMENT: {:?}", self.current_token);
        if self.is_data_type() {
            return Ok(Statement::Assignment(self.parse_assignment()?));
        }
        return Err(format!("Invalid statement"));
    }

    fn parse_number(&self, number: &str) -> ExpressionComponent {
        // FIXME: Bad assumption that this code will never failure
        let value = f64::from_str(&number).unwrap();
        let mut number_type = Type::Int;

        if number.contains('.') {
            number_type = Type::Float;
        }

        ExpressionComponent::Number(number_type, value)
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

    fn has_higher_precedence(&self, current_token: Token, top: Token) -> bool {
        self.get_precedence(current_token) > self.get_precedence(top)
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        println!("PARSING EXPRESSION: {:?}", self.current_token);
        let operators: Vec<Token> = vec![];

        // will contain an AST
        let output: Vec<Token> = vec![];

        while self.current_token != Token::Semicolon {
            match &self.current_token {
                Token::Number(number) => {
                    let number = self.parse_number(&number);
                    output.push(number);
                    self.advance();
                }

                operator if self.is_operator() => {
                    self.advance();
                }

                Token::LeftPar => {
                    operators.push(self.current_token);
                    self.advance();
                }

                Token::RightPar => {
                    let left_parenthesis_found = false;

                    while !operators.is_empty() {
                        let operator = *operators.last().unwrap();
                        if operator == Token::LeftPar {
                            left_parenthesis_found = true;
                            break;
                        }
                        output.push(operator);
                    }

                    if operators.is_empty() && !left_parenthesis_found {
                        return Err("Mismatched parenthesis".to_string());
                    }
                }

                _ => return Err("Invalid expressions".to_string()),
            };

            while !operators.is_empty() {
                let operator = *operators.last().unwrap();
                if operator == Token::LeftPar || operator == Token::RightPar {
                    return Err("Mismatched parenthesis".to_string());
                }
                output.push(operator);
            }
        }
        Ok(())
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

    pub fn generate_ast(&mut self) -> Result<Vec<Statement>, String> {
        self.advance();
        let mut statements: Vec<Statement> = vec![];

        while self.current_token != Token::EOF {
            let statement = self.parse_statement()?;
            statements.push(statement.clone());
            println!("CURRENT STATEMENT: {:?}", statement);
            self.advance();
        }

        Ok(statements)
    }
}
