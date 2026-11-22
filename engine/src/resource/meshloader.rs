use std::fmt::Debug;
use std::str::Lines;

use crate::rendering::drawable::Mesh;
use crate::rendering::shader::ShaderProgram;

#[derive(PartialEq)]
enum OffToken {
    Off,
    Int(u32),
    Float(f32),
}

pub fn mesh_from_off_string(content: &str, _shader: &ShaderProgram) -> Result<Mesh, ParsingError> {
    let mut lines = content.lines();

    let mut line_tokens: Vec<OffToken> = Vec::new();

    line_to_tokens(&mut line_tokens, &mut lines)?;

    if line_tokens[0] != OffToken::Off {
        return Err(ParsingError::new("No file format description"));
    }

    line_to_tokens(&mut line_tokens, &mut lines)?;

    // Ok(Mesh::new(vertices, faces, shader))
    Err(ParsingError::new("Not implemented"))
}

fn line_to_tokens(tokens: &mut Vec<OffToken>, lines: &mut Lines) -> Result<(), ParsingError> {
    let Some(line) = lines.next() else {
        return Err(ParsingError::new("Unexpected end of file"));
    };
    tokens.clear();

    // Assuming comments always are at the start of a line
    while line.is_empty() || line.starts_with("#") {
        if lines.next().is_none() {
            return Err(ParsingError::new("Unexpected end of file"));
        };
    }

    for word in line.split_whitespace() {
        let token = match word {
            "OFF" => OffToken::Off,
            _ if let Ok(v) = word.parse::<u32>() => OffToken::Int(v),
            _ if let Ok(v) = word.parse::<f32>() => OffToken::Float(v),
            _ => Err(ParsingError::new(
                format!("Unrecognized token: {}", word).as_str(),
            ))?,
        };
        tokens.push(token);
    }

    Ok(())
}

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
