mod lexer;
mod parser;
use std::fmt::{Debug, Display};

use lexer::lex::LexerBVH;
use parser::parsing::ParserBVH;

#[derive(Debug)]
pub enum BVHError {
    LexerError(lexer::LexerError),
    ParsingError(parser::ParsingError),
}

pub fn parse(content: String) -> Result<(), BVHError> {
    let tokens = LexerBVH::tokenize(content).map_err(BVHError::LexerError)?;
    let parse = ParserBVH::parse(tokens).map_err(BVHError::ParsingError)?;
    Ok(())
}

pub mod prelude {
    pub use super::parse;
}
