use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    SpecialChar(SpecialCharId),
    Keyword(KeywordId),
    Operator(OperatorId),
    RelOperator(RelOperatorId),
    UnaryOperator(UnaryOperatorId),
    BitwiseOperator(BitwiseOperatorId),
    LogicOperator(LogicOperatorId),
    Number(String),
    Identifier(String),
    EOF,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BitwiseOperatorId {
    And, // &
    Or,  // |
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogicOperatorId {
    And, // &&
    Or,  // ||
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOperatorId {
    Minus,     // -
    Increment, // ++
    Decrement, // --
    Not,       // !
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpecialCharId {
    Colon,
    OpeningPar,
    OpeningCurly,
    OpeningBracket,
    ClosingPar,
    ClosingCurly,
    ClosingBracket,
    Semicolon,
    EqualSign,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeywordId {
    Def,
    If,
    Elif, // Else if
    Else,
    Return,
    Int,
    Float,
    Bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperatorId {
    Plus,
    Minus,
    Mod,
    Divides,
    Times,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RelOperatorId {
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    NotEqual, // !=
    EqualTo,  // ==
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
            ("def", KeywordId::Def),
            ("if", KeywordId::If),
            ("elif", KeywordId::Elif),
            ("else", KeywordId::Else),
            ("return", KeywordId::Return),
            ("bool", KeywordId::Bool),
            ("int", KeywordId::Int),
            ("float", KeywordId::Float),
        ]);

        match keywords.get(identifier) {
            Some(keyword_type) => Token::Keyword(*keyword_type),
            None => Token::Identifier(identifier.to_string()),
        }
    }

    fn get_identifier(&mut self) -> Token {
        let mut identifier = String::from(self.current_char);
        self.advance();

        while (self.current_char.is_alphanumeric() || self.current_char == '_') && !self.is_eof() {
            identifier.push(self.current_char);
            self.advance();
        }

        Token::Identifier(identifier)
    }

    fn get_number(&mut self) -> Token {
        let mut number = String::from(self.current_char);
        self.advance();

        while (self.current_char.is_ascii_digit() || self.current_char == '.') && !self.is_eof() {
            number.push(self.current_char);
            self.advance();
        }

        Token::Number(number)
    }

    fn get_token(&mut self) -> Token {
        self.skip_any_whitespace();

        match self.current_char {
            letter if self.current_char.is_alphabetic() => self.get_identifier(),

            digit if digit.is_ascii_digit() => self.get_number(),

            ':' => self.consume_and_advance(Token::SpecialChar(SpecialCharId::Colon)),

            '(' => self.consume_and_advance(Token::SpecialChar(SpecialCharId::OpeningPar)),

            ')' => self.consume_and_advance(Token::SpecialChar(SpecialCharId::ClosingPar)),

            '{' => self.consume_and_advance(Token::SpecialChar(SpecialCharId::OpeningCurly)),

            '}' => self.consume_and_advance(Token::SpecialChar(SpecialCharId::ClosingCurly)),

            ';' => self.consume_and_advance(Token::SpecialChar(SpecialCharId::Semicolon)),

            '[' => self.consume_and_advance(Token::SpecialChar(SpecialCharId::OpeningBracket)),

            ']' => self.consume_and_advance(Token::SpecialChar(SpecialCharId::ClosingBracket)),

            '=' => {
                self.advance();

                if self.current_char == '=' {
                    return self.consume_and_advance(Token::RelOperator(RelOperatorId::EqualTo));
                }

                return Token::SpecialChar(SpecialCharId::EqualSign);
            }

            '>' => {
                self.advance();

                if self.current_char == '=' {
                    return self.consume_and_advance(Token::RelOperator(
                        RelOperatorId::GreaterThanOrEqual,
                    ));
                }

                return Token::RelOperator(RelOperatorId::GreaterThan);
            }

            '<' => {
                self.advance();

                if self.current_char == '=' {
                    return self
                        .consume_and_advance(Token::RelOperator(RelOperatorId::LessThanOrEqual));
                }

                return Token::RelOperator(RelOperatorId::LessThan);
            }

            '+' => {
                self.advance();

                if self.current_char == '+' {
                    return self
                        .consume_and_advance(Token::UnaryOperator(UnaryOperatorId::Increment));
                }

                return Token::Operator(OperatorId::Plus);
            }

            '-' => {
                self.advance();

                if self.current_char == '-' {
                    return self
                        .consume_and_advance(Token::UnaryOperator(UnaryOperatorId::Decrement));
                }
                return Token::Operator(OperatorId::Minus);
            }

            '!' => {
                self.advance();

                if self.current_char == '=' {
                    return self.consume_and_advance(Token::RelOperator(RelOperatorId::NotEqual));
                }
                return Token::UnaryOperator(UnaryOperatorId::Not);
            }

            '|' => {
                self.advance();

                if self.current_char == '|' {
                    return self.consume_and_advance(Token::LogicOperator(LogicOperatorId::Or));
                }
                return Token::BitwiseOperator(BitwiseOperatorId::Or);
            }

            '&' => {
                self.advance();

                if self.current_char == '&' {
                    return self.consume_and_advance(Token::LogicOperator(LogicOperatorId::And));
                }
                return Token::BitwiseOperator(BitwiseOperatorId::And);
            }

            '/' => self.consume_and_advance(Token::Operator(OperatorId::Divides)),

            '*' => self.consume_and_advance(Token::Operator(OperatorId::Times)),

            '%' => self.consume_and_advance(Token::Operator(OperatorId::Mod)),

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
