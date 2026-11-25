pub mod bvh;
pub mod geometry;
mod lexer;
mod parser;
use std::fmt::Debug;

use lexer::lex::LexerBVH;
use parser::parsing::ParserBVH;

pub use bvh::Bvh;

#[derive(Debug)]
pub enum BVHError {
    LexerError(lexer::LexerError),
    ParsingError(parser::ParsingError),
}

pub fn parse(content: String) -> Result<Bvh, BVHError> {
    let tokens = LexerBVH::tokenize(content).map_err(BVHError::LexerError)?;
    let parse = ParserBVH::parse(tokens).map_err(BVHError::ParsingError)?;
    Ok(Bvh::new(parse))
}
