pub mod bvh_parser;
pub mod clock;
pub mod core;
pub mod rendering;
pub mod resource;

pub mod prelude {
    pub use crate::core::{
        joint::{Joint, JointRecord, JointRecordsQueue},
        skeleton::Skeleton,
        transform::Transform,
        world::{Scheduler, World},
    };

    pub use glam::*;

    pub use crate::bvh_parser::parse;
}
