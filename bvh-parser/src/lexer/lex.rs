pub enum Axis {
    X,
    Y,
    Z,
}

pub enum TokensBVH {
    // Const keywords
    Hierarchy,
    Motion,
    Root,
    Joint,
    End,
    Channels,
    Offsets,
    Frame,
    FrameTime,

    // Channels
    Position(Axis),
    Rotation(Axis),

    // Variables
    Number(u32),
    Real(f32),
    Name(String),

    // Other symbols
    Colon,
    OpeneBrace,
    CloseBrace,
}
