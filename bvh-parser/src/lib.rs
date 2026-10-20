mod lexer;
mod parser;
use lexer::lexer::LexerBVH;

pub fn parse(content: String) {
    let tokens = LexerBVH::tokenize(content);
}

pub mod prelude {
    pub use super::parse;
}
