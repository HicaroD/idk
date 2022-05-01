// TODO(Hícaro): Implement Display trait
#[derive(Debug)]
pub enum Token {
    Def,
    Colon,
    OpeningPar,
    OpeningCurly,
    ClosingPar,
    ClosingCurly,
    Semicolon,
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

    fn get_token(&mut self) -> Token {
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

            ':' => Token::Colon,

            '(' => Token::OpeningPar,

            ')' => Token::ClosingPar,

            '{' => Token::OpeningCurly,

            '}' => Token::ClosingCurly,

            ';' => Token::Semicolon,

            _ => {
                // TODO(Hícaro): When the program finds a whitespace, it crashes, but it
                // should ignore the whitespace
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
                    "def" => tokens.push(Token::Def),
                    _ => tokens.push(Token::Identifier(ident.to_string())),
                }
            } else {
                tokens.push(token);
            }

            // TODO(Hícaro): Improve the condition to break the tokenizer loop
            if self.position == self.source_code.len() {
                break;
            }
            self.advance();
        }
        
        for token in tokens.iter() {
            println!("{:?}", token);
        }

        return tokens;
    }
}
