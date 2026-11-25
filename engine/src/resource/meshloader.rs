use crate::prelude::*;
use std::fmt::Debug;
use std::str::Lines;

use crate::rendering::drawable::Mesh;
use crate::rendering::shader::ShaderProgram;

#[derive(Debug, PartialEq)]
pub enum TokenOff {
    Off,
    Number(usize),
    Real(f32),
    BreakLine,
}

pub fn mesh_from_off_string(content: &str, shader: &ShaderProgram) -> Result<Mesh, ParsingError> {
    let lines = content.lines();

    let mut tokens: Vec<TokenOff> = Vec::new();
    lex(&mut tokens, lines)?;
    let (vertices, faces) = parse(tokens)?;
    println!("{}, {}", vertices[0], faces[0]);

    Ok(Mesh::new(vertices, faces, shader))
}

fn lex(tokens: &mut Vec<TokenOff>, lines: Lines) -> Result<(), ParsingError> {
    for line in lines.into_iter() {
        for value in line.split_whitespace() {
            match value {
                "OFF" => tokens.push(TokenOff::Off),
                _ if let Ok(n) = value.parse::<usize>() => tokens.push(TokenOff::Number(n)),
                _ if let Ok(v) = value.parse::<f32>() => tokens.push(TokenOff::Real(v)),
                _ if value.trim_start().starts_with("#") => break,
                _ => Err(ParsingError::new(
                    format!("Unknown token : {value}").as_str(),
                ))?,
            }
        }
        tokens.push(TokenOff::BreakLine);
    }

    Ok(())
}

fn parse(tokens: Vec<TokenOff>) -> Result<(Vec<Vec3>, Vec<UVec3>), ParsingError> {
    // Iter over tokens
    let mut it = TokenOffIterator::new(tokens);

    // Delete empty lines at the begining of the file
    while let Some(token) = it.peek()
        && *token == TokenOff::BreakLine
    {
        it.next();
    }

    // Look at the header
    match it.next() {
        Some(TokenOff::Off) => (),
        _ => Err(ParsingError::new("OFF header missing"))?,
    }
    it.parse_next_breakline()?;

    // Parse number of vertices, faces and edges
    let n_vertices = it.parse_next_number()?;
    let n_faces = it.parse_next_number()?;
    let mut _n_edges = it.parse_next_number()?;
    it.parse_next_breakline()?;

    // Parse vertices
    let mut vertices = Vec::with_capacity(n_vertices);
    for _ in 0..n_vertices {
        let v = it.parse_vertex()?;
        vertices.push(v);
    }

    // Parse faces
    let mut faces = Vec::with_capacity(n_faces);
    for _ in 0..n_faces {
        let f = it.parse_face()?;
        triangulize(f, &mut faces)?;
    }

    Ok((vertices, faces))
}

// Triangulize a face
fn triangulize(face: Vec<u32>, faces: &mut Vec<UVec3>) -> Result<(), ParsingError> {
    match face.len() {
        ..3 => Err(ParsingError::new(
            "A face cannot contains less than 3 vertices",
        ))?,
        3 => {
            faces.push(UVec3::new(face[0], face[1], face[2]));
        }
        4.. => {
            // Simple face triangulation (no need for specific triangle shape)
            // Given a face [v0, v1, v2, ..., vn]
            // We make triangles [v0, v1, v2], [v0, v2, v3], [v0, v3, v4], ..., [v0, vn-1, vn]

            let iv0 = face[0];

            face[1..face.len() - 1]
                .iter()
                .zip(face[2..face.len()].iter())
                .for_each(|(&iv1, &iv2)| {
                    let face = UVec3::new(iv0, iv1, iv2);
                    faces.push(face);
                });
        }
    }
    Ok(())
}

struct TokenOffIterator {
    it: std::iter::Peekable<std::vec::IntoIter<TokenOff>>,
}

impl TokenOffIterator {
    fn new(tokens: Vec<TokenOff>) -> Self {
        Self {
            it: tokens.into_iter().peekable(),
        }
    }

    fn peek(&mut self) -> Option<&TokenOff> {
        self.it.peek()
    }

    fn next(&mut self) -> Option<TokenOff> {
        self.it.next()
    }

    /// Parse a line as a vertex \
    /// Returns a Vec3A (16 bytes aligned type) for GPU
    fn parse_vertex(&mut self) -> Result<Vec3, ParsingError> {
        let x = self.parse_next_real()?;
        let y = self.parse_next_real()?;
        let z = self.parse_next_real()?;
        self.next_line();
        Ok(Vec3::new(x, y, z))
    }

    /// Parse a line as a face
    fn parse_face(&mut self) -> Result<Vec<u32>, ParsingError> {
        let n = self.parse_next_number()?;
        let mut vertices = Vec::new();
        for _ in 0..n {
            let vi = self.parse_next_number()?;
            vertices.push(vi as u32);
        }
        self.next_line();
        Ok(vertices)
    }

    /// Parse a number whout peeking it
    fn parse_next_number(&mut self) -> Result<usize, ParsingError> {
        match self.next() {
            Some(TokenOff::Number(n)) => Ok(n),
            _ => Err(ParsingError::new("Unexpeced token 0")),
        }
    }

    /// Parse a real wout peeking it
    fn parse_next_real(&mut self) -> Result<f32, ParsingError> {
        match self.next() {
            Some(TokenOff::Real(v)) => Ok(v),
            _ => Err(ParsingError::new("Unexpeced token 1")),
        }
    }

    /// Parse a breakline whout peeking it
    pub fn parse_next_breakline(&mut self) -> Result<(), ParsingError> {
        match self.next() {
            Some(TokenOff::BreakLine) => Ok(()),
            _ => Err(ParsingError::new("Unexpeced token 2")),
        }
    }

    /// Go to the next line and ignore all tokens
    pub fn next_line(&mut self) {
        while let Some(token) = self.next()
            && token != TokenOff::BreakLine
        {}
    }
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
