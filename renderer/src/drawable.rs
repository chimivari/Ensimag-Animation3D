use glam::Vec3;
use std::ffi::c_void;
use std::mem::size_of;
use crate::shader::ShaderProgram;



pub trait Drawable
{
    fn render(&self);
    fn bind_shader_program(&mut self, shader_program: &ShaderProgram);
}

pub struct Lines<'a>
{
    vertices: &'a [Vec3],
    vao: u32,
    vbo: u32,
    gl_shader_program_slot: u32
}

impl<'a> Lines<'a>
{
    pub fn new(vertices: &'a [Vec3], shader_program: &ShaderProgram) -> Lines<'a>
    {
        let mut vao: u32 = u32::MAX;
        let mut vbo: u32 = u32::MAX;

        unsafe
        {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                // I know, but there is NO WAY this fails
                isize::try_from(size_of::<Vec3>() * vertices.len()).unwrap(),
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW);
            // Vertices
            
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (3 * size_of::<f32>()) as i32, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            // Unbind VAO and VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Self {
            vertices,
            vao,
            vbo,
            gl_shader_program_slot: shader_program.gl_program_slot
        }

    }
}

impl Drawable for Lines<'_>
{
    fn render(&self)
    {
        unsafe
        {
            gl::UseProgram(self.gl_shader_program_slot);
            gl::BindVertexArray(self.vao);

            gl::DrawArrays(gl::LINE_LOOP, 0, self.vertices.len() as i32);
        }

    }

    fn bind_shader_program(&mut self, shader_program: &ShaderProgram)
    {
        self.gl_shader_program_slot = shader_program.gl_program_slot;

    }
}

impl Drop for Lines<'_>
{
    fn drop(&mut self)
    {
        if self.vao == u32::MAX { return; }
        
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }

        self.gl_shader_program_slot = u32::MAX;
        self.vao = u32::MAX;
        self.vbo = u32::MAX;
    }

}
