pub mod bvh_parser;
pub mod clock;
pub mod core;
pub mod rendering;
pub mod resource;

pub mod prelude {
    pub use crate::core::{
        joint::{Joint, JointRecord, JointRecordsQueue},
        transform::Transform,
        world::{Scheduler, World},
    };

    pub use glam::Affine3;
    pub use glam::EulerRot;
    pub use glam::Mat4;
    pub use glam::Quat;
    pub use glam::Vec3;
    pub use glam::Vec3 as Vertex;

    pub use crate::bvh_parser::parse;
}
