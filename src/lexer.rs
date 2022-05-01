#[derive(Debug, Clone)]
pub enum Token {
    Def,
    Colon,
    OpeningPar,
    OpeningCurly,
    ClosingPar,
    ClosingCurly,
    Semicolon,
    EqualSign,
    GreaterThan,
    LesserThan,
    If,
    Elif, // Else if
    Else,
    Identifier(String),
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
        while self.current_char.is_whitespace() || self.current_char.is_ascii_whitespace() {
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

            ':' => self.consume_and_advance(Token::Colon),

            '(' => self.consume_and_advance(Token::OpeningPar),

            ')' => self.consume_and_advance(Token::ClosingPar),

            '{' => self.consume_and_advance(Token::OpeningCurly),

            '}' => self.consume_and_advance(Token::ClosingCurly),

            ';' => self.consume_and_advance(Token::Semicolon),

            '=' => self.consume_and_advance(Token::EqualSign),

            '>' => self.consume_and_advance(Token::GreaterThan),

            '<' => self.consume_and_advance(Token::LesserThan),

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
            println!("{:?}", token);

            if let Token::Identifier(ref ident) = token {
                match ident.as_str() {
                    "def"  => tokens.push(Token::Def),
                    "if"   => tokens.push(Token::If),
                    "elif" => tokens.push(Token::Elif),
                    "else" => tokens.push(Token::Else),
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
        
        return tokens;
    }
}
