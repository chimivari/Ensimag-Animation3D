pub mod clock;
pub mod core;
pub mod rendering;
pub mod resource;

pub mod prelude {
    pub use crate::core::joint::Joint;
    pub use crate::core::transform::Transform;
    pub use crate::core::world::Entity;
    pub use crate::core::world::Scheduler;
    pub use crate::core::world::World;

    pub use glam::Affine3A as Affine3;
    pub use glam::EulerRot;
    pub use glam::Mat4;
    pub use glam::Quat;
    pub use glam::Vec3A as Vertex;
    pub use glam::Vec3A as Vec3;
}
