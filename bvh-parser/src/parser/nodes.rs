use crate::geometry::*;
use Channel::*;
use glam::{EulerRot, Mat3A};
use std::{collections::VecDeque, fmt::Debug};

#[derive(Default, Debug)]
pub struct Hierarchy {
    pub roots: Vec<Root>,
}

impl Hierarchy {
    pub fn add_root(&mut self, root: Root) {
        self.roots.push(root);
    }
}

#[derive(Debug)]
pub struct Root {
    pub _name: String,
    pub offset: Vertex,
    pub channels: Vec<Channel>,
    pub children: Vec<Box<dyn HierarchyNode>>,
}

impl Root {
    pub fn new(_name: String, offset: [f32; 3], channels: Vec<Channel>) -> Self {
        if channels.len() != 6 {
            panic!("A Root cannot have less or more than 6 channels");
        }
        match channels[..3] {
            [
                Channel::Position(Axis::X),
                Channel::Position(Axis::Y),
                Channel::Position(Axis::Z),
            ] => (),
            _ => panic!("A root should start with Xposition Yposition Zposition"),
        }
        Self {
            _name,
            offset: Vertex::from_array(offset),
            channels,
            children: Vec::new(),
        }
    }

    pub fn add_child<N: 'static + HierarchyNode>(&mut self, child: N) {
        self.children.push(Box::new(child));
    }

    /// /// Generate the graph of the root
    /// ///
    /// /// - __graph__ contains the index of the parent node (the root is not included)
    /// /// - __vertices__ are in the local space (the root vertex is included)
    /// pub fn generate_graph(
    ///     &mut self,
    ///     vertices: &mut Vec<Vertex>,
    ///     edges: &mut Vec<Edge>,
    ///     graph: &mut Vec<usize>,
    ///     channels: &mut Vec<Option<Vec<Channel>>>,
    /// ) {
    ///     channels.push(Some(
    ///         self.channels
    ///             .take()
    ///             .expect("A graph cannot be generate twice for a same root"),
    ///     ));
    ///     vertices.push(self.offset);
    ///     for child in &mut self.children {
    ///         child.generate_graph(vertices, edges, graph, channels, 0);
    ///     }
    /// }

    pub fn compute_initial_global_pos(&self, vertices: &mut Vec<Vertex>, edges: &mut Vec<Edge>) {
        vertices.push(self.offset);
        for child in &self.children {
            child.compute_initial_global_pos(vertices, edges, 0);
        }
    }

    pub fn compute_global_pos(&self, records: &mut VecDeque<f32>, vertices: &mut Vec<Vertex>) {
        let t = take_position_records(records);
        let r = take_rotation_records(records, &self.channels[3..6]);
        let v = self.offset + t;
        vertices.push(v);
        for child in &self.children {
            child.compute_global_pos(records, vertices, &v, &r);
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Channel {
    Rotation(Axis),
    Position(Axis),
}
impl Channel {
    fn is_position(&self) -> bool {
        matches!(self, Self::Position(..))
    }
}

pub(crate) trait HierarchyNode: Debug {
    fn generate_graph(
        &mut self,
        vertices: &mut Vec<Vertex>,
        edges: &mut Vec<Edge>,
        graph: &mut Vec<usize>,
        channels: &mut Vec<Option<Vec<Channel>>>,
        pvi: usize,
    );

    fn compute_initial_global_pos(
        &self,
        vertices: &mut Vec<Vertex>,
        edges: &mut Vec<Edge>,
        ip: usize,
    );
    fn compute_global_pos(
        &self,
        records: &mut VecDeque<f32>,
        vertices: &mut Vec<Vertex>,
        v_p: &Vertex,
        r_p: &Mat3A,
    );
}

#[derive(Debug)]
pub struct Joint {
    pub _name: String,
    pub offset: Vertex,
    pub channels: Vec<Channel>,
    pub children: Vec<Box<dyn HierarchyNode>>,
}
impl Joint {
    pub fn new(_name: String, offset: [f32; 3], channels: Vec<Channel>) -> Self {
        if channels.len() != 3 {
            panic!("A Joint cannot have less or more than 6 channels");
        }
        if channels[0].is_position() || channels[1].is_position() || channels[2].is_position() {
            panic!("A Joint cannot have position channels");
        }
        Self {
            _name,
            offset: Vertex::from_array(offset),
            channels,
            children: Vec::new(),
        }
    }

    pub fn add_child<N: 'static + HierarchyNode>(&mut self, child: N) {
        self.children.push(Box::new(child));
    }
}
impl HierarchyNode for Joint {
    /// Add the graph of the joint to the root one
    ///
    /// - __pvi__ is the parent vertex index
    /// - __vertices__are in the local space
    fn generate_graph(
        &mut self,
        vertices: &mut Vec<Vertex>,
        edges: &mut Vec<Edge>,
        graph: &mut Vec<usize>,
        channels: &mut Vec<Option<Vec<Channel>>>,
        pvi: usize,
    ) {
        // channels.push(
        //     self.channels
        //         .take()
        //         .expect("A graph cannot be generate twice for a same joint"),
        // );
        // vertices.push(self.offset);
        // // Index of the current node in the graph
        // graph.push(pvi);
        // let cvi = graph.len();
        // edges.push(Edge::new(pvi, cvi));
        // for child in &mut self.children {
        //     child.generate_graph(vertices, edges, graph, channels, cvi);
        // }
    }

    fn compute_initial_global_pos(
        &self,
        vertices: &mut Vec<Vertex>,
        edges: &mut Vec<Edge>,
        ip: usize,
    ) {
        vertices.push(self.offset);
        edges.push(Edge::new(ip, vertices.len()));
        let ip = vertices.len();
        for child in &self.children {
            child.compute_initial_global_pos(vertices, edges, ip);
        }
    }
    fn compute_global_pos(
        &self,
        records: &mut VecDeque<f32>,
        vertices: &mut Vec<Vertex>,
        v_p: &Vertex,
        r_p: &Mat3A,
    ) {
        let v = v_p + r_p * self.offset;
        let r = r_p * take_rotation_records(records, &self.channels);
        vertices.push(v);
        for child in &self.children {
            child.compute_global_pos(records, vertices, &v, &r);
        }
    }
}

#[derive(Debug)]
pub struct End {
    pub _name: String,
    pub offset: Vertex,
}
impl End {
    pub fn new(_name: String, offset: [f32; 3]) -> Self {
        Self {
            _name,
            offset: Vertex::from_array(offset),
        }
    }
}
impl HierarchyNode for End {
    /// Add the graph of the joint to the root one
    ///
    /// - __pvi__ is the parent vertex index
    /// - __vertices__are in the local space
    fn generate_graph(
        &mut self,
        vertices: &mut Vec<Vertex>,
        edges: &mut Vec<Edge>,
        graph: &mut Vec<usize>,
        channels: &mut Vec<Option<Vec<Channel>>>,
        pvi: usize,
    ) {
        channels.push(None);
        vertices.push(self.offset);
        edges.push(Edge::new(pvi, graph.len()));
        graph.push(pvi);
    }

    fn compute_initial_global_pos(
        &self,
        vertices: &mut Vec<Vertex>,
        edges: &mut Vec<Edge>,
        ip: usize,
    ) {
        vertices.push(self.offset);
        edges.push(Edge::new(ip, vertices.len()));
    }

    fn compute_global_pos(
        &self,
        _records: &mut VecDeque<f32>,
        vertices: &mut Vec<Vertex>,
        v_p: &Vertex,
        r_p: &Mat3A,
    ) {
        let v = v_p + r_p * self.offset;
        vertices.push(v);
    }
}

#[derive(Debug)]
pub struct Motion {
    pub frames: usize,
    pub frame_time: f32,
    pub records: VecDeque<f32>,
}
impl Motion {
    pub fn new(frames: usize, frame_time: f32) -> Self {
        Self {
            frames,
            frame_time,
            records: VecDeque::new(),
        }
    }

    pub fn add_record(&mut self, record: f32) {
        self.records.push_back(record);
    }
}

fn take_record(motion: &mut VecDeque<f32>) -> f32 {
    motion
        .pop_front()
        .expect("Missing a record in the MOTION section")
}

fn take_position_records(motion: &mut VecDeque<f32>) -> Vertex {
    Vertex::new(
        take_record(motion),
        take_record(motion),
        take_record(motion),
    )
}

/// Returns the rotation matrix concatenated
///
/// channels is a slice of size 3 (not checked in this method but already checked on parsing)
fn take_rotation_records(motion: &mut VecDeque<f32>, channels: &[Channel]) -> Mat3A {
    let a = take_record(motion).to_radians();
    let b = take_record(motion).to_radians();
    let c = take_record(motion).to_radians();
    match channels {
        [Rotation(Axis::X), Rotation(Axis::Y), Rotation(Axis::Z)] => {
            Mat3A::from_euler(EulerRot::XYZ, a, b, c)
        }
        [Rotation(Axis::X), Rotation(Axis::Z), Rotation(Axis::Y)] => {
            Mat3A::from_euler(EulerRot::XZY, a, b, c)
        }
        [Rotation(Axis::Y), Rotation(Axis::X), Rotation(Axis::Z)] => {
            Mat3A::from_euler(EulerRot::YXZ, a, b, c)
        }
        [Rotation(Axis::Y), Rotation(Axis::Z), Rotation(Axis::X)] => {
            Mat3A::from_euler(EulerRot::YZX, a, b, c)
        }
        [Rotation(Axis::Z), Rotation(Axis::X), Rotation(Axis::Y)] => {
            Mat3A::from_euler(EulerRot::ZXY, a, b, c)
        }
        [Rotation(Axis::Z), Rotation(Axis::Y), Rotation(Axis::X)] => {
            Mat3A::from_euler(EulerRot::ZYX, a, b, c)
        }
        c => panic!("Same rotation defined twice in channels : {c:?}"),
    }
}
