use glam::{EulerRot, Mat3A};
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::ParserBVH;
use crate::geometry::{Axis, Edge, Vertex};
use crate::parser::nodes::Channel::{self, *};

/// A frame with each vertices being in the global space
///
/// Each edge between vertices are specified in the BVH struct
pub struct Frame {
    pub vertices: Vec<Vertex>,
}
impl Frame {
    fn new(vertices: Vec<Vertex>) -> Self {
        Self { vertices }
    }
}

/// Contains all frames for one root
pub struct Root {
    pub frames: Vec<Frame>,
    /// Segments between each vertices of the frame
    pub edges: Vec<Edge>,
}
impl Root {
    fn new(
        root: crate::parser::nodes::Root,
        frame_count: usize,
        records: &mut VecDeque<f32>,
    ) -> Self {
        // Generate a vector with all frames (frame_count + the initial one)
        let mut frames = Vec::with_capacity(frame_count + 1);

        // Generate the initial frame
        let mut vertices = Vec::new();
        let mut edges = Vec::new();
        root.compute_initial_global_pos(&mut vertices, &mut edges);
        frames.push(Frame::new(vertices));

        // Generate other frames
        for _ in 0..frame_count {
            let mut vertices = vec![];
            root.compute_global_pos(records, &mut vertices);
            frames.push(Frame::new(vertices));
        }

        Self { frames, edges }
    }

    /// Save the BVH at the specified location
    ///
    /// Each frame is stored on a line and each line contains a list of XYZ global coordinates
    pub fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        for frame in &self.frames {
            let mut line = frame
                .vertices
                .iter()
                .map(|v| format!("{} {} {}", v.x, v.y, v.z))
                .collect::<Vec<String>>()
                .join(" ");
            line.push('\n');
            file.write_all(&line.into_bytes())?;
        }
        Ok(())
    }
}

/// Contains a BVH for multiple roots
pub struct Bvh {
    /// Time of a frame
    ///
    /// Corresponds to the 'Frame Time' field of the MOTION section
    pub frame_time: f32,
    pub roots: Vec<Root>,
}

impl Bvh {
    pub(crate) fn new(mut content: ParserBVH) -> Self {
        // Generate the graph
        let mut roots = Vec::new();
        for root in content.hierarchy.roots {
            let r = Root::new(root, content.motion.frames, &mut content.motion.records);
            roots.push(r);
        }

        Self {
            frame_time: content.motion.frame_time,
            roots,
        }
    }
}
