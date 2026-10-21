use crate::lexer::lex::LexerBVH;
use crate::lexer::tokens::TokenBVH;

use super::ParsingError;
use super::nodes::*;

#[derive(Debug)]
pub struct ParserBVH {
    hierarchy: Hierarchy,
    motion: Motion,
}

type Peekable = std::iter::Peekable<std::vec::IntoIter<TokenBVH>>;

impl ParserBVH {
    pub fn parse(tokens: LexerBVH) -> Result<Self, ParsingError> {
        let tokens = tokens.into_tokens();
        let mut it: Peekable = tokens.into_iter().peekable();

        // Parse Hierarchy
        if !Self::try_parse_const(&mut it, TokenBVH::Hierarchy) {
            Err(ParsingError::new("Hierarchy missing"))?
        }
        let mut hierarchy = Hierarchy::default();

        loop {
            match Self::parse_root(&mut it) {
                Ok(Some(root)) => hierarchy.add_root(root),
                Ok(None) => break,
                Err(err) => Err(err)?,
            }
        }

        // Parse Motion
        if !Self::try_parse_const(&mut it, TokenBVH::Motion) {
            Err(ParsingError::new("Motion missing"))?
        }
        let motion = Self::parse_motion(&mut it)?;

        Ok(Self { hierarchy, motion })
    }

    fn parse_root(it: &mut Peekable) -> Result<Option<Root>, ParsingError> {
        // Parse ROOT
        if !Self::try_parse_const(it, TokenBVH::Root) {
            return Ok(None);
        }

        // Parse root data
        let name = match Self::try_parse_name(it) {
            Some(name) => name,
            _ => Err(ParsingError::new("ROOT name not found"))?,
        };
        if !Self::try_parse_const(it, TokenBVH::OpenBrace) {
            Err(ParsingError::new("Root misses an open brace"))?
        }
        let offset = Self::parse_offset(it)?;
        let channels = Self::parse_channels(it)?;

        let mut root = Root::new(name, offset, channels);

        // Parse joint nodes
        loop {
            match Self::parse_joint(it) {
                Ok(Some(joint)) => root.add_child(joint),
                Ok(None) => break,
                Err(err) => Err(err)?,
            }
        }

        // Parse end node
        loop {
            match Self::parse_end(it) {
                Ok(Some(end)) => root.add_child(end),
                Ok(None) => break,
                Err(err) => Err(err)?,
            }
        }

        if !Self::try_parse_const(it, TokenBVH::CloseBrace) {
            Err(ParsingError::new("Root misses a closed brace"))?
        }

        Ok(Some(root))
    }

    fn parse_joint(it: &mut Peekable) -> Result<Option<Joint>, ParsingError> {
        // Parse JOINT
        if !Self::try_parse_const(it, TokenBVH::Joint) {
            return Ok(None);
        }
        // Parse joint data
        let name = match Self::try_parse_name(it) {
            Some(name) => name,
            None => Err(ParsingError::new("Joint name not found"))?,
        };
        if !Self::try_parse_const(it, TokenBVH::OpenBrace) {
            Err(ParsingError::new("Joint misses an open brace"))?
        }
        let offset = Self::parse_offset(it)?;
        let channels = Self::parse_channels(it)?;

        let mut joint = Joint::new(name, offset, channels);

        // Parse children joints
        loop {
            match Self::parse_joint(it) {
                Ok(Some(j)) => joint.add_child(j),
                Ok(None) => break,
                Err(err) => Err(err)?,
            }
        }

        // Parse children ends
        loop {
            match Self::parse_end(it) {
                Ok(Some(e)) => joint.add_child(e),
                Ok(None) => break,
                Err(err) => Err(err)?,
            }
        }

        if !Self::try_parse_const(it, TokenBVH::CloseBrace) {
            Err(ParsingError::new("Joint misses a closed brace"))?
        }
        Ok(Some(joint))
    }

    fn parse_end(it: &mut Peekable) -> Result<Option<End>, ParsingError> {
        // Parse End
        if !Self::try_parse_const(it, TokenBVH::End) {
            return Ok(None);
        }
        // Parse end data
        let name = match Self::try_parse_name(it) {
            Some(name) => name,
            None => Err(ParsingError::new("End name not found"))?,
        };
        if !Self::try_parse_const(it, TokenBVH::OpenBrace) {
            Err(ParsingError::new("End misses an open brace"))?
        }
        let offset = Self::parse_offset(it)?;
        let end = End::new(name, offset);

        if !Self::try_parse_const(it, TokenBVH::CloseBrace) {
            Err(ParsingError::new("End misses a closed brace"))?
        }
        Ok(Some(end))
    }

    fn parse_offset(it: &mut Peekable) -> Result<[f32; 3], ParsingError> {
        // Parse OFFSET
        if !Self::try_parse_const(it, TokenBVH::Offset) {
            Err(ParsingError::new("No offset found"))?
        }
        // Parse coordinates
        let mut offset = [0f32; 3];
        for i in 0..3 {
            match Self::try_parse_real(it) {
                Some(v) => offset[i] = v,
                None => Err(ParsingError::new("Offset misses a coordinate"))?,
            }
        }
        Ok(offset)
    }

    fn parse_channels(it: &mut Peekable) -> Result<Vec<Channel>, ParsingError> {
        // Parse CHANNELS
        if !Self::try_parse_const(it, TokenBVH::Channels) {
            Err(ParsingError::new("No channel found"))?
        }
        // Parse nomber of channels
        let n = match Self::try_parse_natural_number(it) {
            Some(n) => n,
            None => Err(ParsingError::new("Number of channels not found"))?,
        };
        // Parse all channels
        let mut channels = Vec::with_capacity(n as usize);
        for _ in 0..n {
            match Self::try_parse_channel(it) {
                Some(c) => channels.push(c),
                None => Err(ParsingError::new("Channel not found"))?,
            }
        }
        Ok(channels)
    }

    fn parse_motion(it: &mut Peekable) -> Result<Motion, ParsingError> {
        // Parse frame count
        if !Self::try_parse_const(it, TokenBVH::Frames) {
            Err(ParsingError::new("Motion doesn't have 'Frames' field"))?
        }
        let frames = match Self::try_parse_natural_number(it) {
            Some(c) => c as usize,
            None => Err(ParsingError::new("'Frames' field misses count"))?,
        };
        // Parse frame time
        if !Self::try_parse_const(it, TokenBVH::Frame) {
            Err(ParsingError::new("Motion doesn't have 'Frame Time' field"))?
        }
        if !Self::try_parse_const(it, TokenBVH::Time) {
            Err(ParsingError::new("'Frame' keyword misses 'Time' keyword"))?
        }
        let frame_time = match Self::try_parse_real(it) {
            Some(ft) => ft,
            None => Err(ParsingError::new("'Frame Time' field misses value"))?,
        };

        let mut motion = Motion::new(frames, frame_time);

        // Parse each record
        loop {
            match Self::try_parse_real(it) {
                Some(r) => motion.add_record(r),
                None => {
                    if it.peek().is_none() {
                        break;
                    } else {
                        Err(ParsingError::new("Motion record not valid"))?
                    }
                }
            }
        }

        Ok(motion)
    }

    /// Returns true if the next token is the expected one.
    ///
    /// Allowed tokens are
    /// - Motion,
    /// - Root,
    /// - Joint,
    /// - End,
    /// - Channels,
    /// - Offset,
    /// - Frames,
    /// - Frame,
    /// - Time,
    /// - FrameTime,
    /// - OpenBrace
    /// - CloseBrace
    ///
    /// The function won't panic if it is another token
    fn try_parse_const(it: &mut Peekable, expected_token: TokenBVH) -> bool {
        let as_expected = it.peek() == Some(&expected_token);
        if as_expected {
            it.next();
        }
        as_expected
    }

    /// Parse a token with a name
    fn try_parse_name(it: &mut Peekable) -> Option<String> {
        match it.peek() {
            Some(TokenBVH::Name(..)) => {
                let name = it.next().unwrap().get_name();
                Some(name)
            }
            _ => None,
        }
    }

    /// Parse a token with a real value
    fn try_parse_real(it: &mut Peekable) -> Option<f32> {
        match it.peek() {
            Some(&TokenBVH::Real(v)) => {
                it.next();
                Some(v)
            }
            _ => None,
        }
    }

    /// Parse a token with a natural number
    fn try_parse_natural_number(it: &mut Peekable) -> Option<u32> {
        match it.peek() {
            Some(&TokenBVH::NaturalNumber(n)) => {
                it.next();
                Some(n)
            }
            _ => None,
        }
    }

    fn try_parse_channel(it: &mut Peekable) -> Option<Channel> {
        match it.peek() {
            Some(&TokenBVH::Position(axis)) => {
                it.next();
                Some(Channel::Position(axis))
            }
            Some(&TokenBVH::Rotation(axis)) => {
                it.next();
                Some(Channel::Rotation(axis))
            }
            _ => None,
        }
    }
}
