use super::LexerError;
use super::tokens::TokenBVH;
use crate::geometry::Axis;

#[derive(Debug)]
pub struct LexerBVH {
    tokens: Vec<TokenBVH>,
}

impl LexerBVH {
    pub fn tokenize(content: String) -> Result<Self, LexerError> {
        let splited_content = content.split_whitespace();
        let mut tokens = vec![];
        for word in splited_content {
            let token = match word {
                "HIERARCHY" => TokenBVH::Hierarchy,
                "ROOT" => TokenBVH::Root,
                "JOINT" => TokenBVH::Joint,
                "End" => TokenBVH::End,
                "OFFSET" => TokenBVH::Offset,
                "CHANNELS" => TokenBVH::Channels,
                "MOTION" => TokenBVH::Motion,
                "Frames:" => TokenBVH::Frames,
                "Frame" => TokenBVH::Frame,
                "Time:" => TokenBVH::Time,

                "Xposition" => TokenBVH::Position(Axis::X),
                "Yposition" => TokenBVH::Position(Axis::Y),
                "Zposition" => TokenBVH::Position(Axis::Z),
                "Xrotation" => TokenBVH::Rotation(Axis::X),
                "Yrotation" => TokenBVH::Rotation(Axis::Y),
                "Zrotation" => TokenBVH::Rotation(Axis::Z),

                "{" => TokenBVH::OpenBrace,
                "}" => TokenBVH::CloseBrace,

                _ if let Ok(v) = word.parse::<u32>() => TokenBVH::NaturalNumber(v),
                _ if let Ok(v) = word.parse::<f32>() => TokenBVH::Real(v),
                _ if word.chars().next().unwrap().is_alphabetic() => {
                    TokenBVH::Name(word.to_string())
                }

                _ => Err(LexerError::new(word.to_string()))?,
            };
            tokens.push(token);
        }
        Ok(Self { tokens })
    }

    /// Returns a vector with all tokens
    pub fn into_tokens(self) -> Vec<TokenBVH> {
        self.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tokenize() {
        let content = std::fs::read_to_string("../inputs/walk1.bvh").unwrap();
        let _ = LexerBVH::tokenize(content);
    }
}
