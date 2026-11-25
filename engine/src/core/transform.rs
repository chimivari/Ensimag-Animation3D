use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Transform {
    pub translation: Vec3,
    pub scale: Vec3,
    pub rotation: Quat,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            scale: Vec3::ONE,
            rotation: Quat::IDENTITY,
        }
    }
}

impl Transform {
    pub fn from_translation(t: Vec3) -> Self {
        Self {
            translation: t,
            ..Default::default()
        }
    }

    pub fn from_rotation(q: Quat) -> Self {
        Self {
            rotation: q,
            ..Default::default()
        }
    }

    pub fn from_translation_rotation(t: Vec3, q: Quat) -> Self {
        Self {
            rotation: q,
            translation: t,
            ..Default::default()
        }
    }

    pub fn rotation_euler(&self) -> Vec3 {
        let (x, y, z) = self.rotation.to_euler(glam::EulerRot::XYZ);
        Vec3::new(x, y, z)
    }

    pub fn to_mat4(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
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
