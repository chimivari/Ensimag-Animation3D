use std::fmt::Debug;

pub mod nodes;
pub mod parsing;

pub struct ParsingError {
    message: String,
}

impl ParsingError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl Debug for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parssing error : {}", self.message)
    }
}
