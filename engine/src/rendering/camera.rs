use glam::{Mat4, Vec3};

pub struct Camera
{
    pub position: Vec3,

    projection: Mat4
}

impl Camera
{
    pub fn new() -> Self
    {
        Self {
            position: Vec3 { x: 0.0, y: 0.0, z: 3.0},
            projection: glam::camera::rh::proj::opengl::perspective(1.0 *0.78539816, 500.0/400.0, 0.1, 100.0)
        }
    }

    pub fn get_projection_mat4(&self) -> &Mat4
    {
        // return &glam::Mat4::IDENTITY;
        return &self.projection;
    }

    pub fn get_view_mat4(&self) -> Mat4
    {
        let up = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
        return glam::camera::rh::view::look_at_mat4(self.position, Vec3::ZERO, up)

    }
    
}
