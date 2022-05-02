#[derive(Debug, Clone)]
pub enum Token {
    SpecialChar(SpecialCharId),
    Keyword(KeywordId),
    Operator(OperatorId), 
    RelOperator(RelOperatorId),
    Number(String),
    Identifier(String),
}

#[derive(Debug, Clone)]
enum SpecialCharId {
    Colon,
    OpeningPar,
    OpeningCurly,
    ClosingPar,
    ClosingCurly,
    Semicolon,
    EqualSign,
}

#[derive(Debug, Clone)]
enum KeywordId {
    Def,
    If,
    Elif, // Else if
    Else,
    Return,
}

#[derive(Debug, Clone)]
enum OperatorId {
    Plus,
    Minus,
    Mod,
    Divides,
    Times,
}

#[derive(Debug, Clone)]
enum RelOperatorId {
    GreaterThan,
    LesserThan,
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

            '=' => self.consume_and_advance(Token::SpecialChar(SpecialCharId::EqualSign)),

            '>' => self.consume_and_advance(Token::RelOperator(RelOperatorId::GreaterThan)),

            '<' => self.consume_and_advance(Token::RelOperator(RelOperatorId::LesserThan)),

            '+'  => self.consume_and_advance(Token::Operator(OperatorId::Plus)),

            '-'  => self.consume_and_advance(Token::Operator(OperatorId::Minus)),

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
                match ident.as_str() {
                    "def" => tokens.push(Token::Keyword(KeywordId::Def)),
                    "if" => tokens.push(Token::Keyword(KeywordId::If)),
                    "elif" => tokens.push(Token::Keyword(KeywordId::Elif)),
                    "else" => tokens.push(Token::Keyword(KeywordId::Else)),
                    "return" => tokens.push(Token::Keyword(KeywordId::Return)),
                    _ => tokens.push(token.clone()),
                }
            } else {
                tokens.push(token);
            }

            // TODO(Hícaro): Improve the condition to break the tokenizer loop
            if self.position == self.source_code.len() {
                break;
            }
        }
        for token in tokens.iter() {
            println!("{:?}", token);
        }

        return tokens;
    }
}
