use crate::prelude::*;

pub struct Joint {
    pub parent: Option<Entity>,
    pub children: Vec<Entity>,
    pub transform: Transform,
}

impl Joint {
    pub fn new(parent: Option<Entity>, transform: Transform) -> Self {
        Self {
            parent,
            transform,
            children: Vec::new(),
        }
    }

    pub fn add_children(&mut self, entity: Entity) {
        self.children.push(entity);
    }
}
