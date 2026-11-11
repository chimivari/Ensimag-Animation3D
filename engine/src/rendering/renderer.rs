use std::rc::Rc;
use std::cell::RefCell;

use crate::rendering::drawable::Drawable;
use crate::rendering::shader::{ShaderConfig, ShaderProgram};
use crate::rendering::glfwwindow::GLFWWindow;
use crate::rendering::camera::Camera;

use glfw::Context;


pub struct Renderer
{
    stop_request: bool,
    window: GLFWWindow,
    shader_programs: Vec<ShaderProgram>,
    pub drawables: Vec<Rc<RefCell<dyn Drawable>>>
}

impl Renderer
{
    // TODO: Return actual errors
    pub fn try_new(width: u32, height: u32, title: &str) -> Result<Self, ()>
    {
        Ok(Self { 
            stop_request: false,
            window: GLFWWindow::try_new(width, height, title)?,
            shader_programs: Vec::new(),
            drawables: Vec::new()
        })
    }

    pub fn create_shader_program(&mut self, vertex_shader_config: ShaderConfig, fragment_shader_config: ShaderConfig) -> Result<&ShaderProgram, String>
    {
        let index: usize = self.shader_programs.len();

        self.shader_programs.push(
            ShaderProgram::try_new(vertex_shader_config, fragment_shader_config)?);

        Ok(&self.shader_programs[index])
    }

    pub fn stop_requested(&self) -> bool
    {
        return self.stop_request || self.window.window.should_close();
    }

    pub fn render(&mut self, camera: &Camera)
    {
        unsafe
        {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        for shader in &mut self.shader_programs
        {
            shader.update_camera_uniforms(camera);
        }

        for drawable in &self.drawables
        {
            drawable.borrow().render();
        }

        self.window.window.swap_buffers();
    }

    // TODO: Modify to return events (abstracted from glfw)
    pub fn flush_events(&mut self)
    {
        self.window.glfw.poll_events();
        glfw::flush_messages(&self.window.events);
    }
}

impl Drop for Renderer
{
    fn drop(&mut self)
    {
        // Force drop on drawables to prevent opengl segfault
        self.drawables.clear();
        // Force drop on shader programs too
        self.shader_programs.clear();
    }
    
}
