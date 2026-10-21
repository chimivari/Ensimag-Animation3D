use std::fmt::Debug;

use crate::lexer::tokens::Axis;

#[derive(Default, Debug)]
pub struct Hierarchy {
    roots: Vec<Root>,
}
impl Hierarchy {
    pub fn add_root(&mut self, root: Root) {
        self.roots.push(root);
    }
}

#[derive(Debug)]
pub struct Root {
    name: String,
    offset: [f32; 3],
    channels: Vec<Channel>,
    children: Vec<Box<dyn HierarchyNode>>,
}
impl Root {
    pub fn new(name: String, offset: [f32; 3], channels: Vec<Channel>) -> Self {
        Self {
            name,
            offset,
            channels,
            children: Vec::new(),
        }
    }

    pub fn add_child<N: 'static + HierarchyNode>(&mut self, child: N) {
        self.children.push(Box::new(child));
    }
}

#[derive(Debug)]
pub enum Channel {
    Rotation(Axis),
    Position(Axis),
}

trait HierarchyNode: Debug {}
impl HierarchyNode for Joint {}
impl HierarchyNode for End {}

#[derive(Debug)]
pub struct Joint {
    name: String,
    offset: [f32; 3],
    channels: Vec<Channel>,
    children: Vec<Box<dyn HierarchyNode>>,
}
impl Joint {
    pub fn new(name: String, offset: [f32; 3], channels: Vec<Channel>) -> Self {
        Self {
            name,
            offset,
            channels,
            children: Vec::new(),
        }
    }

    pub fn add_child<N: 'static + HierarchyNode>(&mut self, child: N) {
        self.children.push(Box::new(child));
    }
}

#[derive(Debug)]
pub struct End {
    name: String,
    offset: [f32; 3],
}
impl End {
    pub fn new(name: String, offset: [f32; 3]) -> Self {
        Self { name, offset }
    }
}

#[derive(Debug)]
pub struct Motion {
    frames: usize,
    frame_time: f32,
    records: Vec<f32>,
}
impl Motion {
    pub fn new(frames: usize, frame_time: f32) -> Self {
        Self {
            frames,
            frame_time,
            records: Vec::new(),
        }
    }

    pub fn add_record(&mut self, record: f32) {
        self.records.push(record);
    }
}
