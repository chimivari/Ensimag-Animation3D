#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, PartialEq)]
pub enum TokenBVH {
    // Const keywords
    Hierarchy,
    Motion,
    Root,
    Joint,
    End,
    Channels,
    Offset,
    Frames,
    Frame,
    Time,

    // Channels
    Position(Axis),
    Rotation(Axis),

    // Variables
    NaturalNumber(u32),
    Real(f32),
    Name(String),

    // Other symbols
    OpenBrace,
    CloseBrace,
}

impl TokenBVH {
    /// Returns the string contaned in TokenBVH::Name
    ///
    /// Panics if it is not the right token
    pub fn get_name(self) -> String {
        match self {
            TokenBVH::Name(name) => name,
            _ => panic!("Cannot get the name of this token : {self:?}"),
        }
    }

    /// Returns the u32 contaned in TokenBVH::NaturalNumber
    ///
    /// Panics if it is not the right token
    pub fn get_natural_number(self) -> u32 {
        match self {
            TokenBVH::NaturalNumber(n) => n,
            _ => panic!("Cannot get the name of this token : {self:?}"),
        }
    }

    /// Returns the f32 contaned in TokenBVH::Real
    ///
    /// Panics if it is not the right token
    pub fn get_real(self) -> f32 {
        match self {
            TokenBVH::Real(v) => v,
            _ => panic!("Cannot get the name of this token : {self:?}"),
        }
    }
}
