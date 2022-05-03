#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone)]
pub enum BitwiseOperatorId {
    And, // &
    Or,  // |
}

#[derive(Debug, Clone)]
pub enum LogicOperatorId {
    And, // &&
    Or,  // ||
}

#[derive(Debug, Clone)]
pub enum UnaryOperatorId {
    Minus,      // -
    Increment,  // ++
    Decrement,  // --
    Not,        // !
}

#[derive(Debug, Clone)]
pub enum SpecialCharId {
    Colon,
    OpeningPar,
    OpeningCurly,
    ClosingPar,
    ClosingCurly,
    Semicolon,
    EqualSign,
}

#[derive(Debug, Clone)]
pub enum KeywordId {
    Def,
    If,
    Elif, // Else if
    Else,
    Return,
}

#[derive(Debug, Clone)]
pub enum OperatorId {
    Plus,
    Minus,
    Mod,
    Divides,
    Times,
}

#[derive(Debug, Clone)]
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
}

impl Lexer {
    pub fn new(source_code: Vec<char>) -> Self {
        Self {
            source_code,
            position: 0,
            current_char: '0',
        }
    }

    fn advance(&mut self) {
        if self.position < self.source_code.len() {
            self.current_char = self.source_code[self.position];
            self.position += 1;
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
        match identifier {
            "def" => Token::Keyword(KeywordId::Def),
            "if" => Token::Keyword(KeywordId::If),
            "elif" => Token::Keyword(KeywordId::Elif),
            "else" => Token::Keyword(KeywordId::Else),
            "return" => Token::Keyword(KeywordId::Return),
            _ => Token::Identifier(identifier.to_string()),
        }
    }

    fn get_token(&mut self) -> Token {
        self.skip_any_whitespace();

        match self.current_char {
            letter if self.current_char.is_alphabetic() => {
                let mut identifier = String::from(letter);
                self.advance();

                while self.current_char.is_alphanumeric() || self.current_char == '_' {
                    identifier.push(self.current_char);
                    self.advance();
                }

                Token::Identifier(identifier)
            }

            digit if digit.is_ascii_digit() => {
                let mut number = String::from(digit);
                self.advance();

                while self.current_char.is_ascii_digit() || self.current_char == '.' {
                    number.push(self.current_char);
                    self.advance();
                }

                Token::Number(number)
            }

            ':' => self.consume_and_advance(Token::SpecialChar(SpecialCharId::Colon)),

            '(' => self.consume_and_advance(Token::SpecialChar(SpecialCharId::OpeningPar)),

            ')' => self.consume_and_advance(Token::SpecialChar(SpecialCharId::ClosingPar)),

            '{' => self.consume_and_advance(Token::SpecialChar(SpecialCharId::OpeningCurly)),

            '}' => self.consume_and_advance(Token::SpecialChar(SpecialCharId::ClosingCurly)),

            ';' => self.consume_and_advance(Token::SpecialChar(SpecialCharId::Semicolon)),

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
                    return self.consume_and_advance(Token::RelOperator(RelOperatorId::GreaterThanOrEqual));
                }

                return Token::RelOperator(RelOperatorId::GreaterThan);
            }

            '<' => {
                self.advance();
                
                if self.current_char == '=' {
                    return self.consume_and_advance(Token::RelOperator(RelOperatorId::LessThanOrEqual));
                }

                return Token::RelOperator(RelOperatorId::LessThan);
            }

            '+'  => {
                self.advance();

                if self.current_char == '+' {
                    return self.consume_and_advance(Token::UnaryOperator(UnaryOperatorId::Increment));
                }

                return Token::Operator(OperatorId::Plus);
            }

            '-'  => {
                self.advance();

                if self.current_char == '-' {
                    return self.consume_and_advance(Token::UnaryOperator(UnaryOperatorId::Decrement));
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

            '/'  => self.consume_and_advance(Token::Operator(OperatorId::Divides)),

            '*'  => self.consume_and_advance(Token::Operator(OperatorId::Times)),

            '%'  => self.consume_and_advance(Token::Operator(OperatorId::Mod)),

            _ => {
                eprintln!("Error: Invalid token '{:?}'", self.current_char);
                std::process::exit(1);
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        self.advance();
        loop {
            let token = self.get_token();

            if let Token::Identifier(ref ident) = token {
                tokens.push(self.classify_identifier(ident));
            } else {
                tokens.push(token);
            }

            // TODO(Hícaro): Improve the condition to break the tokenizer loop
            if self.position == self.source_code.len() {
                break;
            }
        }
        return tokens;
    }
}
