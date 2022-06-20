use crate::ast::Type;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Token {
    Keyword(KeywordId),

    Number(String),
    StringValue(String),
    Identifier(String),

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

    EOF,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum KeywordId {
    Fn, // Function declaration
    If,
    Elif, // Else if
    Else,
    Return,
    Int,
    Float,
    Bool,
    StringKeyword,
}

impl KeywordId {
    pub fn as_type(&self) -> Result<Type, String> {
        match &self {
            KeywordId::Int => Ok(Type::Int),
            KeywordId::Float => Ok(Type::Float),
            KeywordId::Bool => Ok(Type::Bool),
            KeywordId::StringKeyword => Ok(Type::StringType),
            keyword => Err(format!("Can't parse type: {:?}", keyword)),
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
        let keywords: HashMap<&str, KeywordId> = HashMap::from([
            ("fn", KeywordId::Fn),
            ("if", KeywordId::If),
            ("elif", KeywordId::Elif),
            ("else", KeywordId::Else),
            ("return", KeywordId::Return),
            ("bool", KeywordId::Bool),
            ("int", KeywordId::Int),
            ("float", KeywordId::Float),
            ("string", KeywordId::StringKeyword),
        ]);

        match keywords.get(identifier) {
            Some(keyword_type) => Token::Keyword(*keyword_type),
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

        while self.current_char.is_ascii_digit() || self.current_char == '.' {
            number.push(self.current_char);
            self.advance();
        }

        Token::Number(number)
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

                return Token::EqualSign;
            }

            '>' => {
                self.advance();

                if self.current_char == '=' {
                    return self.consume_and_advance(Token::GreaterThanOrEqual);
                }

                return Token::GreaterThan;
            }

            '<' => {
                self.advance();

                if self.current_char == '=' {
                    return self.consume_and_advance(Token::LessThanOrEqual);
                }

                return Token::LessThan;
            }

            '+' => {
                self.advance();

                if self.current_char == '+' {
                    return self.consume_and_advance(Token::Increment);
                }

                return Token::Plus;
            }

            '-' => {
                self.advance();

                if self.current_char == '-' {
                    return self.consume_and_advance(Token::Decrement);
                }
                return Token::Minus;
            }

            '!' => {
                self.advance();

                if self.current_char == '=' {
                    return self.consume_and_advance(Token::NotEqual);
                }
                return Token::Not;
            }

            '|' => {
                self.advance();

                if self.current_char == '|' {
                    return self.consume_and_advance(Token::LogicOr);
                }
                return Token::BitwiseOr;
            }

            '&' => {
                self.advance();

                if self.current_char == '&' {
                    return self.consume_and_advance(Token::LogicAnd);
                }
                return Token::BitwiseAnd;
            }

            '/' => self.consume_and_advance(Token::Divides),

            '*' => {
                self.advance();

                if self.current_char == '*' {
                    return self.consume_and_advance(Token::Power);
                }
                return Token::Times;
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
        tokens.push(Token::EOF);
        return tokens;
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
            Token::Keyword(KeywordId::Int),
            Token::Identifier("variable_name".to_string()),
            Token::EqualSign,
            Token::Number("12".to_string()),
            Token::Semicolon,
            Token::EOF,
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
            Token::EOF,
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
            Token::EOF,
        ];

        assert_eq!(tokens, expected_result);
    }

    #[test]
    fn test_string() {
        let input = "\"my string here\"\n".chars().collect::<Vec<char>>();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        let expected_result: Vec<Token> =
            vec![Token::StringValue("my string here".to_string()), Token::EOF];

        assert_eq!(tokens, expected_result);
    }
}
