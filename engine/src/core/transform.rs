use glam::{Mat4, Quat, Vec3A};
use std::rc::Rc;

pub struct Transform
{
    pub parent: Option<Rc<Transform>>,
    pub position: Vec3A,
    pub scale: Vec3A,
    pub rotation: Quat

}

impl Transform
{
    pub fn rotation_euler(&self) -> Vec3A
    {
        let (x, y, z) = self.rotation.to_euler(glam::EulerRot::XYZ);
        return Vec3A::new(x, y, z);
    }

    pub fn to_mat4(&self) -> Mat4
    {
        return Mat4::from_scale_rotation_translation(self.scale.to_vec3(), self.rotation, self.position.to_vec3());
    }

    pub fn to_global_mat4(&self) -> Mat4
    {
        let m = self.to_mat4();

        match &self.parent
        {
            None => m,
            // Should be correct order, i.e. when transforming vertex x, the equivalent
            // transformation is applied: Parent * Local * x = Parent * (Local * x)
            Some(p) => p.parent.as_ref().unwrap().to_global_mat4() * m
        }
    }
}
