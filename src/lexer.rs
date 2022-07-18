use crate::ast::Type;
use std::collections::{HashMap, HashSet};

#[derive(PartialEq)]
pub enum Associativity {
    Left,
    Right,
    Undefined,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Token {
    IntNumber(String),
    FloatNumber(String),
    StringValue(String),

    Identifier(String),

    // Keywords
    KeywordFn,     // fn
    KeywordIf,     // if
    KeywordElif,   // elif
    KeywordElse,   // else
    KeywordReturn, // return
    KeywordInt,    // int
    KeywordFloat,  // float
    KeywordBool,   // bool
    KeywordString, // string

    // Special characters
    Colon,
    LeftBracket,
    RightBracket,
    LeftCurly,
    RightCurly,
    LeftPar,
    RightPar,
    Semicolon,
    EqualSign,
    Comma,

    // Operator
    Plus,
    Minus,
    Mod,
    Divides,
    Times,
    Power, // **

    // Relational operator
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    NotEqual,
    EqualTo,

    // Unary operator
    Increment, // ++
    Decrement, // --
    Not,       // !

    // Logic operator
    LogicAnd, // &&
    LogicOr,  // ||

    // Bitwise operator
    BitwiseAnd, // &
    BitwiseOr,  // |

    Eof,
}

impl Token {
    // FIXME: is a HashSet too much? Could I just use a match statement?
    pub fn is_operator(&self) -> bool {
        let operators: HashSet<Token> = HashSet::from([
            Token::Plus,
            Token::Minus,
            Token::Mod,
            Token::Divides,
            Token::Times,
            Token::Power,
        ]);
        operators.get(self).is_some()
    }

    pub fn is_data_type_keyword(&self) -> bool {
        let data_types: HashSet<Token> = HashSet::from([
            Token::KeywordInt,
            Token::KeywordFloat,
            Token::KeywordBool,
            Token::KeywordString,
        ]);

        data_types.get(self).is_some()
    }

    pub fn as_type(&self) -> Result<Type, String> {
        match self {
            Token::KeywordInt => Ok(Type::Int),
            Token::KeywordFloat => Ok(Type::Float),
            Token::KeywordBool => Ok(Type::Bool),
            Token::KeywordString => Ok(Type::StringType),
            token => Err(format!("Can't parse type: {:?}", token)),
        }
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Token::IntNumber(_) | Token::FloatNumber(_))
    }

    pub fn get_associativity(&self) -> Associativity {
        match self {
            Token::Plus | Token::Minus | Token::Times | Token::Divides => Associativity::Left,
            _ => Associativity::Undefined,
        }
    }

    pub fn has_higher_precedence(&self, second_token: &Token) -> bool {
        self.get_precedence() > second_token.get_precedence()
    }

    pub fn has_same_precedence(&self, second_token: &Token) -> bool {
        self.get_precedence() == second_token.get_precedence()
    }

    pub fn get_precedence(&self) -> i8 {
        match self {
            Token::Plus => 1,
            Token::Minus => 1,
            Token::Times => 2,
            Token::Divides => 2,
            _ => -1,
        }
    }
}

pub struct Lexer {
    source_code: Vec<char>,
    position: usize,
    current_char: char,
    is_end_of_file: bool,
}

impl Lexer {
    pub fn new(source_code: Vec<char>) -> Self {
        Self {
            source_code,
            position: 0,
            current_char: '0',
            is_end_of_file: false,
        }
    }

    fn is_eof(&self) -> bool {
        self.position >= self.source_code.len()
    }

    fn advance(&mut self) {
        if !self.is_eof() {
            self.current_char = self.source_code[self.position];
            self.position += 1;
        } else {
            self.is_end_of_file = true;
        }
    }

    fn skip_any_whitespace(&mut self) {
        while self.current_char.is_whitespace() {
            self.advance();
        }
    }

    fn consume_and_advance(&mut self, token: Token) -> Token {
        self.advance();
        token
    }

    fn classify_identifier(&self, identifier: &str) -> Token {
        let keywords: HashMap<&str, Token> = HashMap::from([
            ("fn", Token::KeywordFn),
            ("if", Token::KeywordIf),
            ("elif", Token::KeywordElif),
            ("else", Token::KeywordElse),
            ("return", Token::KeywordReturn),
            ("bool", Token::KeywordBool),
            ("int", Token::KeywordInt),
            ("float", Token::KeywordFloat),
            ("string", Token::KeywordString),
        ]);

        match keywords.get(identifier) {
            Some(keyword_type) => keyword_type.clone(),
            None => Token::Identifier(identifier.to_string()),
        }
    }

    fn get_identifier(&mut self) -> Token {
        let mut identifier = String::from(self.current_char);
        self.advance();

        while self.current_char.is_alphanumeric() || self.current_char == '_' {
            identifier.push(self.current_char);
            self.advance();
        }

        Token::Identifier(identifier)
    }

    fn get_number(&mut self) -> Token {
        let mut number = String::from(self.current_char);
        self.advance();

        let mut is_float = false;

        while self.current_char.is_ascii_digit() || self.current_char == '.' {
            if self.current_char == '.' {
                is_float = true;
            }
            number.push(self.current_char);
            self.advance();
        }

        if is_float {
            Token::FloatNumber(number)
        } else {
            Token::IntNumber(number)
        }
    }

    fn get_string(&mut self) -> Token {
        self.advance();
        let mut string = String::new();

        println!("READING STRING: {}", self.current_char);
        while self.current_char != '"' {
            string.push(self.current_char);
            self.advance();
        }

        self.advance();
        Token::StringValue(string)
    }

    fn get_token(&mut self) -> Token {
        self.skip_any_whitespace();

        match self.current_char {
            letter if letter.is_alphabetic() => self.get_identifier(),

            digit if digit.is_ascii_digit() => self.get_number(),

            '"' => self.get_string(),

            ':' => self.consume_and_advance(Token::Colon),

            '(' => self.consume_and_advance(Token::LeftPar),

            ')' => self.consume_and_advance(Token::RightPar),

            '{' => self.consume_and_advance(Token::LeftCurly),

            '}' => self.consume_and_advance(Token::RightCurly),

            ';' => self.consume_and_advance(Token::Semicolon),

            ',' => self.consume_and_advance(Token::Comma),

            '[' => self.consume_and_advance(Token::LeftBracket),

            ']' => self.consume_and_advance(Token::RightBracket),

            '=' => {
                self.advance();

                if self.current_char == '=' {
                    return self.consume_and_advance(Token::EqualTo);
                }

                Token::EqualSign
            }

            '>' => {
                self.advance();

                if self.current_char == '=' {
                    return self.consume_and_advance(Token::GreaterThanOrEqual);
                }

                Token::GreaterThan
            }

            '<' => {
                self.advance();

                if self.current_char == '=' {
                    return self.consume_and_advance(Token::LessThanOrEqual);
                }

                Token::LessThan
            }

            '+' => {
                self.advance();

                if self.current_char == '+' {
                    return self.consume_and_advance(Token::Increment);
                }

                Token::Plus
            }

            '-' => {
                self.advance();

                if self.current_char == '-' {
                    return self.consume_and_advance(Token::Decrement);
                }
                Token::Minus
            }

            '!' => {
                self.advance();

                if self.current_char == '=' {
                    return self.consume_and_advance(Token::NotEqual);
                }
                Token::Not
            }

            '|' => {
                self.advance();

                if self.current_char == '|' {
                    return self.consume_and_advance(Token::LogicOr);
                }
                Token::BitwiseOr
            }

            '&' => {
                self.advance();

                if self.current_char == '&' {
                    return self.consume_and_advance(Token::LogicAnd);
                }
                Token::BitwiseAnd
            }

            '/' => self.consume_and_advance(Token::Divides),

            '*' => {
                self.advance();

                if self.current_char == '*' {
                    return self.consume_and_advance(Token::Power);
                }
                Token::Times
            }

            '%' => self.consume_and_advance(Token::Mod),

            _ => {
                eprintln!("Error: Invalid token '{:?}'", self.current_char);
                std::process::exit(1);
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        self.advance();

        while !self.is_eof() {
            let token = self.get_token();

            if let Token::Identifier(ref ident) = token {
                tokens.push(self.classify_identifier(ident));
            } else {
                tokens.push(token);
            }

            if self.is_end_of_file {
                break;
            }
        }
        tokens.push(Token::Eof);
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_declaration_tokens() {
        let input = "int variable_name = 12;\n".chars().collect::<Vec<char>>();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        let expected_result: Vec<Token> = vec![
            Token::KeywordInt,
            Token::Identifier("variable_name".to_string()),
            Token::EqualSign,
            Token::IntNumber("12".to_string()),
            Token::Semicolon,
            Token::Eof,
        ];

        assert_eq!(tokens, expected_result);
    }

    #[test]
    fn test_operators() {
        let input = "> < >= <= == != / * + - ** ++ -- ! && || & |\n"
            .chars()
            .collect::<Vec<char>>();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        let expected_result: Vec<Token> = vec![
            Token::GreaterThan,
            Token::LessThan,
            Token::GreaterThanOrEqual,
            Token::LessThanOrEqual,
            Token::EqualTo,
            Token::NotEqual,
            Token::Divides,
            Token::Times,
            Token::Plus,
            Token::Minus,
            Token::Power,
            Token::Increment,
            Token::Decrement,
            Token::Not,
            Token::LogicAnd,
            Token::LogicOr,
            Token::BitwiseAnd,
            Token::BitwiseOr,
            Token::Eof,
        ];

        assert_eq!(tokens, expected_result);
    }

    #[test]
    fn test_special_characters() {
        let input = "{ } [ ] ( ) = , ; :".chars().collect::<Vec<char>>();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        let expected_result: Vec<Token> = vec![
            Token::LeftCurly,
            Token::RightCurly,
            Token::LeftBracket,
            Token::RightBracket,
            Token::LeftPar,
            Token::RightPar,
            Token::EqualSign,
            Token::Comma,
            Token::Semicolon,
            Token::Colon,
            Token::Eof,
        ];

        assert_eq!(tokens, expected_result);
    }

    #[test]
    fn test_string() {
        let input = "\"my string here\"\n".chars().collect::<Vec<char>>();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        let expected_result: Vec<Token> =
            vec![Token::StringValue("my string here".to_string()), Token::Eof];

        assert_eq!(tokens, expected_result);
    }
}
