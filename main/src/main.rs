use glam::Vec3;

use bvh_parser::prelude::*;

use renderer::shader::{ShaderConfig, ShaderSourceType};
use renderer::renderer::Renderer;
use renderer::drawable::Lines;

#[warn(unused)]
fn main_parser()
{
    let bvh_file = std::fs::read_to_string("inputs/Example1.bvh").unwrap();
    let bvh = parse(bvh_file).unwrap();
    let _ = bvh.roots[0].save("output2.txt");
}

#[warn(unused)]
fn main_renderer_test()
{
    let vertex_shader_path = "./resources/shaders/base.vert";
    let fragment_shader_path = "./resources/shaders/base.frag";

    let vertices: [Vec3; 3] = [
        Vec3::new( 0.0,  0.5, 0.0),
        Vec3::new( 0.5, -0.5, 0.0),
        Vec3::new(-0.5, -0.5, 0.0)
    ];

    
    let mut renderer = Renderer::try_new(500, 400, "Kinematic animations").unwrap();
    
    let vertex_shader = ShaderConfig {
        name: "default vertex shader".to_string(),
        source_type: ShaderSourceType::FileName(vertex_shader_path.to_string())
    };

    let fragment_shader = ShaderConfig {
        name: "default fragment shader".to_string(),
        source_type: ShaderSourceType::FileName(fragment_shader_path.to_string())
    };

    let shader_program = renderer.create_shader_program(vertex_shader, fragment_shader).unwrap();

    let lines = Box::new(Lines::new(&vertices, shader_program));

    renderer.drawables.push(lines);

    while !renderer.stop_requested()
    {
        renderer.render();

        renderer.flush_events();
    }

}

fn main() {
    // main_parser();
    main_renderer_test();
}
