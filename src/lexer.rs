enum Token {
    Identifier(String),
    Def,
}

pub struct Lexer {
    source_code: String,
}

impl Lexer {
    pub fn new(source_code: String) -> Self {
        Self {
            source_code
        }
    }

    pub fn tokenize(&self) {
        let mut source_code = self.source_code.chars();
        let mut tokens: Vec<Token> = vec![];

        while let Some(character) = source_code.next() {
            // TODO(Hícaro): Tokenize file
            println!("{character}");
        }
    }
}
