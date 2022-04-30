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

    fn get_token(&self) -> Token {
        // TODO(Hícaro): Get current token
        return Token::Identifier("Something".to_string());
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        self.advance();
        loop {
            println!("{:?}", self.current_char);

            // TODO(Hícaro): Improve the condition to break the tokenizer loop
            if self.position == self.source_code.len() {
                break;
            }
            self.advance();
        }

        return tokens;
    }
}
