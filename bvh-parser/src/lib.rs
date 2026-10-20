mod lexer;
mod parser;
use lexer::lex::LexerBVH;

pub fn parse(content: String) {
    let tokens = LexerBVH::tokenize(content);
    println!("{tokens:?}");
}

pub mod prelude {
    pub use super::parse;
}
