extern crate glfw;
extern crate gl;

use glam::Vec3;

use crate::drawable::Lines;
use crate::renderer::Renderer;

use crate::shader::{ShaderConfig, ShaderSourceType};

mod glfwwindow;
pub mod shader;
pub mod renderer;
pub mod drawable;

pub fn main_renderer_test() {

    let vertex_shader_source =
"#version 330 core
layout (location = 0) in vec3 aPos;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}\0";

    let fragment_shader_source =
"#version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}\0";

    let vertices: [Vec3; 3] = [
        Vec3::new( 0.0,  0.5, 0.0),
        Vec3::new( 0.5, -0.5, 0.0),
        Vec3::new(-0.5, -0.5, 0.0)
    ];

    
    let mut renderer = Renderer::try_new(500, 400, "Kinematic animations").unwrap();
    
    let vertex_shader = ShaderConfig {
        name: "default vertex shader".to_string(),
        source_type: ShaderSourceType::String(vertex_shader_source.to_string())
    };

    let fragment_shader = ShaderConfig {
        name: "default fragment shader".to_string(),
        source_type: ShaderSourceType::String(fragment_shader_source.to_string())
    };

    let shader_program = renderer.create_shader_program(vertex_shader, fragment_shader).unwrap();

    println!("AAA 1");

    let lines = Box::new(Lines::new(&vertices, shader_program));
    println!("AAA 2");

    renderer.drawables.push(lines);

    println!("AAA 3");
    while !renderer.stop_requested()
    {
        renderer.render();

        renderer.flush_events();

    }

}
