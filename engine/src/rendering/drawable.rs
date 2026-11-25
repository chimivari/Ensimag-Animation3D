use crate::prelude::*;
use crate::rendering::shader::ShaderProgram;
use std::ffi::c_void;
use std::mem::size_of;

pub trait Drawable: Component {
    fn render(&self);
    // WARN: Should be removed, a shader shouldn't be bound to a model, as a model can be reused
    // using multiple transforms, and also shaders
    // fn bind_shader_program(&mut self, shader_program: &ShaderProgram);
}

pub struct Lines {
    gl_shader_program_slot: u32,
    vao: u32,
    vbo: u32,
    vertices: Box<[Vec3]>,
}

impl Lines {
    pub fn new(vertices: &[Vec3], shader_program: &ShaderProgram) -> Self {
        let mut vao: u32 = u32::MAX;
        let mut vbo: u32 = u32::MAX;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                // I know, but there is NO WAY this fails
                isize::try_from(size_of::<Vec3>() * vertices.len()).unwrap(),
                vertices.as_ptr() as *const c_void,
                gl::DYNAMIC_DRAW,
            );
            // Vertices

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * size_of::<f32>()) as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // Unbind VAO
            // Should VBO be unbinded ??
            gl::BindVertexArray(0);
        }

        Self {
            vertices: Box::from(vertices),
            vao,
            vbo,
            gl_shader_program_slot: shader_program.gl_program_slot,
        }
    }

    pub fn update_all_vertices(&mut self, new_values: &[Vec3]) {
        self.vertices = Box::from(new_values);
        unsafe {
            // gl::BindVertexArray(self.vao);
            // gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                // I know, but there is NO WAY this fails
                isize::try_from(size_of::<Vec3>() * self.vertices.len()).unwrap(),
                self.vertices.as_ptr() as *const c_void,
                gl::DYNAMIC_DRAW,
            );
        }
    }

    pub fn update_vertices(&mut self, indices: &[usize], new_values: &[Vec3]) {}

    pub fn update_vertex(&mut self, index: usize, vertex: &Vec3) {}

    pub fn bind_shader_program(&mut self, shader_program: &ShaderProgram) {
        self.gl_shader_program_slot = shader_program.gl_program_slot;
    }
}

impl Component for Lines {}

impl Drawable for Lines {
    fn render(&self) {
        unsafe {
            gl::UseProgram(self.gl_shader_program_slot);

            gl::BindVertexArray(self.vao);
            gl::EnableVertexAttribArray(0);

            gl::DrawArrays(gl::LINES, 0, self.vertices.len() as i32);

            // gl::DisableVertexAttribArray(0);
            // gl::BindVertexArray(0);
        }
    }
}

impl Drop for Lines {
    fn drop(&mut self) {
        if self.vao == u32::MAX {
            return;
        }

        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }

        self.gl_shader_program_slot = u32::MAX;
        self.vao = u32::MAX;
        self.vbo = u32::MAX;
    }
}

pub struct Mesh {
    gl_shader_program_slot: u32,
    vao: u32,
    vbo: u32,
    ebo: u32,
    n_elements: u32,
    // Mesh information is stored on the GPU
}

impl Mesh {
    pub fn new(vertices: Vec<Vec3>, faces: Vec<UVec3>, shader: &ShaderProgram) -> Self {
        let mut vao: u32 = u32::MAX;
        let mut vbo: u32 = u32::MAX;
        let mut ebo: u32 = u32::MAX;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                isize::try_from(size_of::<Vec3>() * vertices.len()).unwrap(),
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * size_of::<f32>()) as i32,
                std::ptr::null(),
            );

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                isize::try_from(size_of::<UVec3>() * faces.len()).unwrap(),
                faces.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::EnableVertexAttribArray(0);

            // unbind vao
            gl::BindVertexArray(0);
        }

        Self {
            n_elements: (faces.len() * 3) as u32,
            gl_shader_program_slot: shader.gl_program_slot,
            vao,
            vbo,
            ebo,
        }
    }
}

impl Component for Mesh {}

impl Drawable for Mesh {
    fn render(&self) {
        unsafe {
            gl::UseProgram(self.gl_shader_program_slot);

            gl::BindVertexArray(self.vao);
            gl::EnableVertexAttribArray(0);

            gl::DrawElements(
                gl::TRIANGLES,
                self.n_elements as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        if self.vao == u32::MAX {
            return;
        }

        self.gl_shader_program_slot = u32::MAX;
        self.vao = u32::MAX;
        self.vbo = u32::MAX;

        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}
