use std::fmt::Debug;

pub mod lex;
pub mod tokens;

pub struct LexerError {
    token: String,
}

impl LexerError {
    fn new(token: String) -> Self {
        Self { token }
    }
}

impl Debug for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown token : {}", self.token)
    }
}
