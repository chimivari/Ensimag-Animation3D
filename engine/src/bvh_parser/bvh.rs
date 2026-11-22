use std::collections::VecDeque;

use crate::bvh_parser::ParserBVH;
use crate::bvh_parser::parser::nodes::BvhRoot;
use crate::prelude::*;

/// Contains a BVH for multiple roots
pub struct Bvh {
    /// Time of a frame
    ///
    /// Corresponds to the 'Frame Time' field of the MOTION section
    frame_time: f32,
    root: BvhRoot,
    records: VecDeque<f32>,
}

impl Bvh {
    pub(crate) fn new(content: ParserBVH) -> Self {
        let frame_time = content.motion.frame_time;
        let ParserBVH { motion, hierarchy } = content;
        Self {
            frame_time,
            root: hierarchy.root,
            records: motion.records,
        }
    }

    pub fn frame_time(&self) -> f32 {
        self.frame_time
    }

    pub fn generate_components(&mut self) -> Vec<(Joint<usize>, JointRecordsQueue)> {
        // Compute initial pose
        let mut joints = Vec::new();
        self.root.compute_initial_local_pose(&mut joints);

        // Compute next poses until records is empty
        let mut joint_records = vec![JointRecordsQueue::default(); joints.len()];
        while !self.records.is_empty() {
            self.root
                .compute_next_local_pose(&mut joint_records, &mut self.records);
        }

        joints.into_iter().zip(joint_records).collect()
    }
}
