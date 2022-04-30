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
}
