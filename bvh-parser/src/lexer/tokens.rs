use crate::geometry::Axis;

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
}
