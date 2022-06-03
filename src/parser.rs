use crate::lexer::*;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current_token: Token,
    pub position: usize,
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

    pub fn generate_ast(&mut self) {
        self.advance();

        while self.current_token != Token::EOF {
            println!("{:?}", self.current_token);
            self.advance();
        }
    }
}
