use crate::core::Component;
use crate::prelude::*;
use crate::rendering::shader::ShaderProgram;
use std::ffi::c_void;
use std::mem::size_of;

pub trait Drawable : Component {
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

impl Component for Lines{}

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

#[repr(C)]
struct AugmentedVertex
{
    vertex: Vec3,
    normal: Vec3
}

fn compute_vec3_running_average(avg: &Vec3, x: &Vec3, n: u8) -> Vec3
{
    (((avg * n as f32) + x) / ((n + 1) as f32))
}

fn compute_mesh_per_vertex_normals(vertices: &Vec<Vec3>, faces: &Vec<UVec3>) -> Vec<AugmentedVertex>
{
    let mut res: Vec<AugmentedVertex> = vertices.into_iter()
        .map(|v| AugmentedVertex { vertex: *v, normal: Vec3 { x: 1.0, y : 0.5, z: 0.0} })
        .collect();
    // Count of all faces touching a vertex
    let mut count: Vec<u8> = vec![0; vertices.len()];

    for face in faces
    {
        let u = vertices[face.x as usize];
        let v = vertices[face.y as usize];
        let w = vertices[face.z as usize];

        let face_normal = (u - v).cross(u - w).normalize();

        // Update normal average for all vertices
        res[face.x as usize].normal = compute_vec3_running_average(
            &res[face.x as usize].normal, &face_normal, count[face.x as usize]);
        count[face.x as usize] += 1;

        res[face.y as usize].normal = compute_vec3_running_average(
            &res[face.y as usize].normal, &face_normal, count[face.y as usize]);
        count[face.y as usize] += 1;

        res[face.z as usize].normal = compute_vec3_running_average(
            &res[face.z as usize].normal, &face_normal, count[face.z as usize]);
        count[face.z as usize] += 1;
    }

    res
}

impl Mesh {
    pub fn new(vertices: Vec<Vec3>, faces: Vec<UVec3>, shader: &ShaderProgram) -> Self {
        // compile_error!("Replace u16 to UVec3 in the pipeline");
        let mut vao: u32 = u32::MAX;
        let mut vbo: u32 = u32::MAX;
        let mut ebo: u32 = u32::MAX;

        let vertices = compute_mesh_per_vertex_normals(&vertices, &faces);

        println!("- {}", size_of::<AugmentedVertex>());

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                isize::try_from(size_of::<AugmentedVertex>() * vertices.len()).unwrap(),
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<AugmentedVertex>() as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::TRUE,
                size_of::<AugmentedVertex>() as i32,
                std::mem::offset_of!(AugmentedVertex, normal) as *const c_void
            );
            gl::EnableVertexAttribArray(1);

            println!("{}", std::mem::offset_of!(AugmentedVertex, normal));

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                isize::try_from(size_of::<UVec3>() * faces.len()).unwrap(),
                faces.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );


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

impl Component for Mesh{}

impl Drawable for Mesh {
    fn render(&self) {
        unsafe {
            gl::UseProgram(self.gl_shader_program_slot);

            gl::BindVertexArray(self.vao);
            // gl::EnableVertexAttribArray(0);
            // gl::EnableVertexAttribArray(1);

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
