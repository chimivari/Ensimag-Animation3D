use crate::bvh_parser::geometry::*;
use crate::prelude::*;
use Channel::*;
use std::{collections::VecDeque, fmt::Debug};

/// Only supports one Root
#[derive(Debug)]
pub struct Hierarchy {
    pub root: BvhRoot,
}

impl Hierarchy {
    pub fn new(root: BvhRoot) -> Self {
        Self { root }
    }
}

#[derive(Debug)]
pub struct BvhRoot {
    pub _name: String,
    pub offset: Vec3,
    pub channels: Vec<Channel>,
    pub children: Vec<Box<dyn HierarchyNode>>,
}

impl BvhRoot {
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
            offset: Vec3::from_array(offset),
            channels,
            children: Vec::new(),
        }
    }

    pub fn add_child<N: 'static + HierarchyNode>(&mut self, child: N) {
        self.children.push(Box::new(child));
    }

    pub fn compute_initial_local_pose(&self, joints: &mut Vec<Joint<usize>>) {
        let joint = Joint::<usize>::new(None, Transform::from_translation(self.offset));
        joints.push(joint);

        for child in &self.children {
            child.compute_initial_local_pose(joints, 0);
        }
    }

    pub fn compute_next_local_pose(
        &self,
        joint_records: &mut [JointRecordsQueue],
        records: &mut VecDeque<f32>,
    ) {
        let translaton = take_position_records(records);
        let rotation = take_rotation_records(records, &self.channels[3..]);

        let root_translation = JointRecord::from_translation(translaton);
        joint_records[0].push(root_translation);

        if self.children.is_empty() {
            return;
        }

        let joint_rotation = JointRecord::from_rotation(rotation);
        joint_records[1].push(joint_rotation);

        let mut n = 1;

        for child in &self.children {
            child.compute_next_local_pose(joint_records, records, &mut n);
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
    fn compute_initial_local_pose(&self, joints: &mut Vec<Joint<usize>>, ip: usize);

    /// n is the index of the last JointRecords added in joint_records
    fn compute_next_local_pose(
        &self,
        joint_records: &mut [JointRecordsQueue],
        records: &mut VecDeque<f32>,
        n: &mut usize,
    );
}

#[derive(Debug)]
pub struct BvhJoint {
    pub _name: String,
    pub offset: Vec3,
    pub channels: Vec<Channel>,
    pub children: Vec<Box<dyn HierarchyNode>>,
}
impl BvhJoint {
    pub fn new(_name: String, offset: [f32; 3], channels: Vec<Channel>) -> Self {
        if channels.len() != 3 {
            panic!("A Joint cannot have less or more than 6 channels");
        }
        if channels[0].is_position() || channels[1].is_position() || channels[2].is_position() {
            panic!("A Joint cannot have position channels");
        }
        Self {
            _name,
            offset: Vec3::from_array(offset),
            channels,
            children: Vec::new(),
        }
    }

    pub fn add_child<N: 'static + HierarchyNode>(&mut self, child: N) {
        self.children.push(Box::new(child));
    }
}
impl HierarchyNode for BvhJoint {
    fn compute_initial_local_pose(&self, joints: &mut Vec<Joint<usize>>, ip: usize) {
        let joint = Joint::<usize>::new(Some(ip), Transform::from_translation(self.offset));

        let id = joints.len();
        joints[ip].add_children(id);

        joints.push(joint);

        for child in &self.children {
            child.compute_initial_local_pose(joints, id);
        }
    }

    fn compute_next_local_pose(
        &self,
        joint_records: &mut [JointRecordsQueue],
        records: &mut VecDeque<f32>,
        n: &mut usize,
    ) {
        *n += 1;
        let rotation = take_rotation_records(records, &self.channels);
        joint_records[*n].push(JointRecord::from_rotation(rotation));

        for child in &self.children {
            child.compute_next_local_pose(joint_records, records, n);
        }
    }
}

#[derive(Debug)]
pub struct BvhEnd {
    pub _name: String,
    pub offset: Vec3,
}
impl BvhEnd {
    pub fn new(_name: String, offset: [f32; 3]) -> Self {
        Self {
            _name,
            offset: Vec3::from_array(offset),
        }
    }
}
impl HierarchyNode for BvhEnd {
    fn compute_initial_local_pose(&self, joints: &mut Vec<Joint<usize>>, ip: usize) {
        let id = joints.len();
        let joint = Joint::new(Some(ip), Transform::from_translation(self.offset));
        joints.push(joint);
        joints[ip].children.push(id);
    }

    fn compute_next_local_pose(
        &self,
        _joint_records: &mut [JointRecordsQueue],
        _records: &mut VecDeque<f32>,
        _n: &mut usize,
    ) {
    }
}

#[derive(Debug)]
pub struct Motion {
    pub _frames: usize,
    pub frame_time: f32,
    pub records: VecDeque<f32>,
}
impl Motion {
    pub fn new(_frames: usize, frame_time: f32) -> Self {
        Self {
            _frames,
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

fn take_position_records(motion: &mut VecDeque<f32>) -> Vec3 {
    Vec3::new(
        take_record(motion),
        take_record(motion),
        take_record(motion),
    )
}

/// Returns the rotation matrix concatenated
///
/// channels is a slice of size 3 (not checked in this method but already checked on parsing)
fn take_rotation_records(motion: &mut VecDeque<f32>, channels: &[Channel]) -> Quat {
    let a = take_record(motion).to_radians();
    let b = take_record(motion).to_radians();
    let c = take_record(motion).to_radians();
    match channels {
        [Rotation(Axis::X), Rotation(Axis::Y), Rotation(Axis::Z)] => {
            Quat::from_euler(EulerRot::XYZ, a, b, c)
        }
        [Rotation(Axis::X), Rotation(Axis::Z), Rotation(Axis::Y)] => {
            Quat::from_euler(EulerRot::XZY, a, b, c)
        }
        [Rotation(Axis::Y), Rotation(Axis::X), Rotation(Axis::Z)] => {
            Quat::from_euler(EulerRot::YXZ, a, b, c)
        }
        [Rotation(Axis::Y), Rotation(Axis::Z), Rotation(Axis::X)] => {
            Quat::from_euler(EulerRot::YZX, a, b, c)
        }
        [Rotation(Axis::Z), Rotation(Axis::X), Rotation(Axis::Y)] => {
            Quat::from_euler(EulerRot::ZXY, a, b, c)
        }
        [Rotation(Axis::Z), Rotation(Axis::Y), Rotation(Axis::X)] => {
            Quat::from_euler(EulerRot::ZYX, a, b, c)
        }
        c => panic!("Same rotation defined twice in channels : {c:?}"),
    }
}
