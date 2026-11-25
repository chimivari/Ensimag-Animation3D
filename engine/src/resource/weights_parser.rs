pub struct WeightsParser {
    /// Name of each joint
    pub joint_names: Vec<String>,
    /// All weights of the vertex i could be found :
    /// ```
    /// weights[vertices_alias[i] as usize..vertices_alias.get(i + 1).map(|&v| v as usize).unwrap_or(weights.len())]
    /// ```
    /// Same for joint_ids
    pub vertices_alias: Vec<u32>,
    /// Contains all weights for every vertex
    pub weights: Vec<f32>,
    /// Used to match each weight to the corresponding joint
    pub joint_ids: Vec<u32>,
}

impl WeightsParser {
    pub fn new(file_content: String) -> Result<Self, ParsingError> {
        let (tokens, expected_vertex_count) = lex(file_content)?;
        parse(tokens, expected_vertex_count)
    }
}

#[derive(Debug)]
enum WeightsToken {
    Id,
    Joint(String),
    #[allow(unused)]
    Number(u32),
    Real(f32),
    BreakLine,
}

fn lex(file_content: String) -> Result<(Vec<WeightsToken>, usize), ParsingError> {
    let mut tokens = Vec::new();
    let mut expected_vertex_count = 0;

    for line in file_content.lines() {
        for value in line.split_whitespace() {
            let token = match value {
                "id" => WeightsToken::Id,
                _ if let Ok(v) = value.parse::<u32>() => WeightsToken::Number(v),
                _ if let Ok(v) = value.parse::<f32>() => WeightsToken::Real(v),
                _ => WeightsToken::Joint(value.to_string()),
            };
            tokens.push(token);
        }
        tokens.push(WeightsToken::BreakLine);
        expected_vertex_count += 1;
    }

    expected_vertex_count -= 1;

    Ok((tokens, expected_vertex_count))
}

fn parse(
    tokens: Vec<WeightsToken>,
    expected_vertex_count: usize,
) -> Result<WeightsParser, ParsingError> {
    let mut it = WeightsIterator::new(tokens);

    it.parse_id()?;

    // Parse joint names
    let mut joint_names = Vec::new();
    loop {
        match it.parse_joint_or_breakline() {
            Ok(Some(v)) => joint_names.push(v),
            Ok(None) => break,
            Err(v) => Err(v)?,
        }
    }

    if expected_vertex_count == 0 {
        return Ok(WeightsParser {
            joint_names,
            vertices_alias: Vec::new(),
            weights: Vec::new(),
            joint_ids: Vec::new(),
        });
    }

    // Parse vertex weights
    let n = joint_names.len() as u32;
    let mut vertices_alias = Vec::with_capacity(expected_vertex_count);
    let mut weights = Vec::with_capacity(expected_vertex_count);
    let mut joint_ids = Vec::with_capacity(expected_vertex_count);

    loop {
        if it.peek_number() {
            it.next();
        } else {
            break;
        }
        vertices_alias.push(weights.len() as u32);
        for i in 0..n {
            let weight = it.parse_real()?;
            if weight > 0.0 {
                weights.push(weight);
                joint_ids.push(i);
            }
        }
        if it.peek_breakline() {
            it.next();
        } else {
            break;
        }
    }

    Ok(WeightsParser {
        joint_names,
        vertices_alias,
        weights,
        joint_ids,
    })
}

struct WeightsIterator {
    it: std::iter::Peekable<std::vec::IntoIter<WeightsToken>>,
}

impl WeightsIterator {
    fn new(tokens: Vec<WeightsToken>) -> Self {
        Self {
            it: tokens.into_iter().peekable(),
        }
    }

    fn peek(&mut self) -> Option<&WeightsToken> {
        self.it.peek()
    }

    fn next(&mut self) -> Option<WeightsToken> {
        self.it.next()
    }

    fn parse_id(&mut self) -> Result<(), ParsingError> {
        match self.it.next() {
            Some(WeightsToken::Id) => Ok(()),
            _ => ParsingError::new("Missing the field 'id'"),
        }
    }

    fn parse_joint_or_breakline(&mut self) -> Result<Option<String>, ParsingError> {
        match self.it.next() {
            Some(WeightsToken::Joint(v)) => Ok(Some(v)),
            Some(WeightsToken::BreakLine) => Ok(None),
            v => ParsingError::new(format!("Unexpected token(1) : {v:?}")),
        }
    }

    fn parse_real(&mut self) -> Result<f32, ParsingError> {
        match self.it.next() {
            Some(WeightsToken::Real(v)) => Ok(v),
            v => ParsingError::new(format!("Unexpected token(3) : {v:?}")),
        }
    }

    fn peek_number(&mut self) -> bool {
        matches!(self.peek(), Some(WeightsToken::Number(_)))
    }

    fn peek_breakline(&mut self) -> bool {
        matches!(self.peek(), Some(WeightsToken::BreakLine))
    }
}

#[derive(Debug)]
pub struct ParsingError {
    pub message: String,
}

impl ParsingError {
    fn new<C, P: AsRef<str>>(message: P) -> Result<C, Self> {
        Err(Self {
            message: message.as_ref().to_string(),
        })
    }
}
