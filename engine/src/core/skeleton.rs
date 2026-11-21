use crate::prelude::*;

pub struct Skeleton {
    pub joints: Vec<Joint>,
}

impl Skeleton {
    pub fn new_empty() -> Self {
        Self { joints: Vec::new() }
    }

    pub fn add_joint(&mut self, joint: Joint) {
        self.joints.push(joint)
    }
}
