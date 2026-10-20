use std::fmt::Debug;

use super::lexer::tokens::TokenBVH;

pub struct ParsingError {
    token: TokenBVH,
}

impl ParsingError {
    pub fn new(token: TokenBVH) -> Self {
        Self { token }
    }
}

impl Debug for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parsing error : token no expected {:?}", self.token)
    }
}
