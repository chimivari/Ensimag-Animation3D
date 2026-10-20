pub mod lexer;
pub mod tokens;

pub struct LexerError {
    token: String,
}

impl LexerError {
    fn new(token: String) -> Self {
        Self { token }
    }
}
