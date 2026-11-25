use crate::prelude::*;

use crate::core::Component;

use crate::core::world::World;
use crate::rendering::camera::Camera;
use crate::rendering::drawable::{Drawable, Lines, Mesh};
use crate::rendering::glfwwindow::GLFWWindow;
use crate::rendering::shader::{ShaderConfig, ShaderProgram};

use glfw::Context;

pub struct Renderer {
    stop_request: bool,
    window: GLFWWindow,
    shader_programs: Vec<ShaderProgram>,
}

impl Renderer {
    // TODO: Return actual errors
    pub fn try_new(width: u32, height: u32, title: &str) -> Result<Self, ()> {
        Ok(Self {
            stop_request: false,
            window: GLFWWindow::try_new(width, height, title)?,
            shader_programs: Vec::new(),
        })
    }

    pub fn create_shader_program(
        &mut self,
        vertex_shader_config: ShaderConfig,
        fragment_shader_config: ShaderConfig,
    ) -> Result<&ShaderProgram, String> {
        let index: usize = self.shader_programs.len();

        self.shader_programs.push(ShaderProgram::try_new(
            vertex_shader_config,
            fragment_shader_config,
        )?);

        Ok(&self.shader_programs[index])
    }

    pub fn stop_requested(&self) -> bool {
        return self.stop_request || self.window.window.should_close();
    }

    pub fn start(&self) {
        unsafe {
            // Depth testing & buffer
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
            // Enable culling
            gl::Enable(gl::CULL_FACE);
            // Setup alpha blending
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::ClearDepth(1.0);
        }
    }

    pub fn render(&self, world: &World) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        if let Some(cameras) = world.iter_component::<Camera>() {
            for (_, camera) in cameras {
                for shader in &self.shader_programs {
                    shader.update_camera_uniforms(camera);
                }
            }
        }

        if let Some(drawables) = world.iter_component::<Lines>() {
            for (_, drawable) in drawables {
                drawable.render();
            }
        }

        if let Some(drawables) = world.iter_component::<Mesh>() {
            for (_, drawable) in drawables {
                drawable.render();
            }
        }
    }

    pub fn swap_buffers(&mut self) {
        self.window.window.swap_buffers();
    }

    // TODO: Modify to return events (abstracted from glfw)
    pub fn flush_events(&mut self) {
        self.window.glfw.poll_events();
        glfw::flush_messages(&self.window.events);
    }
}

impl Component for Renderer {}

impl Drop for Renderer {
    fn drop(&mut self) {
        // Force drop on shader programs too
        self.shader_programs.clear();
    }
}
