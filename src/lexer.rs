#[derive(Debug)]
pub enum Token {
    Def,
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

                while self.current_char.is_alphabetic() || self.current_char == '_' {
                    identifier.push(self.current_char);
                    self.advance();
                }

                Token::Identifier(identifier)
            }

            _ => {
                eprintln!("Error: Invalid token");
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

            // TODO(Hícaro): Improve the condition to break the tokenizer loop
            if self.position == self.source_code.len() {
                break;
            }
            self.advance();
        }

        return tokens;
    }
}
