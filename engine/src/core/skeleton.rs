use std::collections::VecDeque;

use crate::{bvh_parser::Bvh, core::Component, prelude::*};

#[derive(Debug)]
pub struct Skeleton {
    joints: Vec<Joint<usize>>,
    joint_records: Vec<JointRecordsQueue>,
}

impl Skeleton {
    pub fn new(mut bvh: Bvh) -> Self {
        let (joints, joint_records): (Vec<_>, Vec<_>) =
            bvh.generate_components().into_iter().unzip();
        assert_eq!(joints.len(), joint_records.len());

        Self {
            joints,
            joint_records,
        }
    }

    // Compute the next frame
    //
    // Returns `false` if no frame remains
    pub fn compute_next_frame(&mut self) -> bool {
        let mut transforms = VecDeque::new();

        for record_queue in &mut self.joint_records {
            if let Some(record) = record_queue.pop() {
                transforms.push_back(record);
            } else {
                break;
            }
        }

        if transforms.is_empty() {
            return false;
        }
        assert_eq!(transforms.len(), self.joints.len());

        for joint in &mut self.joints {
            let record = transforms.pop_front().unwrap();
            joint.transform = record.transform;
        }

        true
    }

    /// Returns the remaining frame count of the root
    pub fn remaining_frame_count(&self) -> usize {
        self.joint_records
            .first()
            .map(|queue| queue.len())
            .unwrap_or(0)
    }
}

impl Component for Skeleton {}
