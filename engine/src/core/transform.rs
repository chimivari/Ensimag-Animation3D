use super::Component;
use crate::prelude::*;

#[derive(Debug)]
pub struct Transform {
    pub parent: Option<Entity>,
    pub children: Vec<Entity>,
    pub translation: Vec3,
    pub scale: Vec3,
    pub rotation: Quat,
}

impl Transform {
    pub fn rotation_euler(&self) -> Vec3 {
        let (x, y, z) = self.rotation.to_euler(glam::EulerRot::XYZ);
        Vertex::new(x, y, z)
    }

    pub fn to_mat4(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            self.scale.to_vec3(),
            self.rotation,
            self.translation.to_vec3(),
        )
    }

    pub fn set_translation(&mut self, translation: Vec3) {
        self.translation = translation;
    }

    // pub fn to_global_mat4(&self) -> Mat4 {
    //     let m = self.to_mat4();
    //
    //     match &self.parent {
    //         None => m,
    //         // Should be correct order, i.e. when transforming vertex x, the equivalent
    //         // transformation is applied: Parent * Local * x = Parent * (Local * x)
    //         Some(p) => {
    //             p.borrow()
    //                 .parent
    //                 .as_ref()
    //                 .unwrap()
    //                 .borrow()
    //                 .to_global_mat4()
    //                 * m
    //         }
    //     }
    // }
}

impl Component for Transform {}
