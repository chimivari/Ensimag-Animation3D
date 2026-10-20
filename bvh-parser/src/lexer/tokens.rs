#[derive(Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug)]
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
    FrameTime,

    // Channels
    Position(Axis),
    Rotation(Axis),

    // Variables
    NaturalNumber(u32),
    Real(f32),
    Name(String),

    // Other symbols
    OpeneBrace,
    CloseBrace,
}
