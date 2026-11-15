use std::fmt::Debug;
use std::str::Lines;

use crate::rendering::drawable::Mesh;
use crate::rendering::shader::ShaderProgram;

#[derive(PartialEq)]
enum OFFToken
{
    OFF,
    INT(u32),
    FLOAT(f32)
}

pub fn mesh_from_off_string(content: &str, shader: &ShaderProgram) -> Result<Mesh, ParsingError>
{
    let lines = content.lines();

    let mut line_tokens: Vec<OFFToken>;

    line_to_tokens(&mut line_tokens, &mut lines)?;

    if line_tokens[0] != OFFToken::OFF {
        return Err(ParsingError::new("No file format description"));
    }

    line_to_tokens(&mut line_tokens, &mut lines)?;
    

    

    Ok(Mesh::new(vertices, faces, shader))
}

fn line_to_tokens(tokens: &mut Vec<OFFToken>, lines: &mut Lines) -> Result<(), ParsingError>
{
    let Some(line) = lines.next() else { return Err(ParsingError::new("Unexpected end of file")); };
    tokens.clear();

    // Assuming comments always are at the start of a line
    while line.len() == 0 || line.starts_with("#")
    {
        let Some(line) = lines.next() else { return Err(ParsingError::new("Unexpected end of file")); };
    }

    for word in line.split_whitespace()
    {
        let token = match word {
           "OFF" => OFFToken::OFF,
           _ if let Ok(v) = word.parse::<u32>() => OFFToken::INT(v),
           _ if let Ok(v) = word.parse::<f32>() => OFFToken::FLOAT(v),
           _ => Err(ParsingError::new(format!("Unrecognized token: {}", word)))?
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

