use std::collections::VecDeque;

use super::IEntity;
use crate::prelude::*;

#[derive(Debug)]
pub struct Joint<E: IEntity> {
    pub parent: Option<E>,
    pub children: Vec<E>,
    pub transform: Transform,
}

impl<E: IEntity> Joint<E> {
    pub fn new(parent: Option<E>, transform: Transform) -> Self {
        Self {
            parent,
            transform,
            children: Vec::new(),
        }
    }

    pub fn new_with_children(parent: Option<E>, transform: Transform, children: Vec<E>) -> Self {
        Self {
            parent,
            transform,
            children,
        }
    }

    pub fn add_children(&mut self, entity: E) {
        self.children.push(entity);
    }

    pub fn compute_next_pose(&mut self, joint_record: JointRecord) {}
}

impl<E: IEntity + 'static> Component for Joint<E> {}

#[derive(Clone, Debug)]
pub struct JointRecord {
    pub transform: Transform,
}

impl JointRecord {
    pub fn from_translation(t: Vec3) -> Self {
        Self {
            transform: Transform::from_translation(t),
        }
    }

    pub fn from_rotation(quat: Quat) -> Self {
        Self {
            transform: Transform::from_rotation(quat),
        }
    }

    pub fn from_translation_rotation(t: Vec3, q: Quat) -> Self {
        Self {
            transform: Transform::from_translation_rotation(t, q),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct JointRecordsQueue {
    record_queue: VecDeque<JointRecord>,
}

impl JointRecordsQueue {
    pub fn push(&mut self, joint_record: JointRecord) {
        self.record_queue.push_back(joint_record);
    }

    pub fn pop(&mut self) -> Option<JointRecord> {
        self.record_queue.pop_front()
    }

    pub fn len(&self) -> usize {
        self.record_queue.len()
    }
}

impl Component for JointRecordsQueue {}
