#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

/// Contains the indices of two vertices in each frame
pub struct Edge {
    /// Index of the first vertex in each frame
    pub iv1: usize,
    /// INdex of the second vertex in each frame
    pub iv2: usize,
}

impl Edge {
    pub(crate) fn new(iv1: usize, iv2: usize) -> Self {
        Self { iv1, iv2 }
    }
}

pub type Vertex = glam::Vec3A;
