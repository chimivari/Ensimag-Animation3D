use engine::clock::Clock;
use engine::rendering::camera::Camera;
use glam::Vec3;

use bvh_parser::prelude::*;

use engine::rendering::shader::{ShaderConfig, ShaderSourceType};
use engine::rendering::renderer::Renderer;
use engine::rendering::drawable::Lines;

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

    // Quad
    let vertices: [Vec3; 8] = [
        Vec3::new(-5., -5.0, 0.0),
        Vec3::new( 5., -5.0, 0.0),
        Vec3::new( 5., -5.0, 0.0),
        Vec3::new( 5.,  5.0, 0.0),
        Vec3::new( 5.,  5.0, 0.0),
        Vec3::new(-5.,  5.0, 0.0),
        Vec3::new(-5.,  5.0, 0.0),
        Vec3::new(-5., -5.0, 0.0),
    ];

    // let lines = Box::new(Lines::new(&vertices, shader_program));
    // renderer.drawables.push(lines);
    
    let bvh_file = std::fs::read_to_string("inputs/walk1.bvh").unwrap();
    let bvh = parse(bvh_file).unwrap();
    
    // \_(-_-)_/
    let skelett_vertices = bvh.roots[0].frames[1].vertices.clone();
    // let skelett_vertices: Vec<Vec3> = skelett_vertices.into_iter().map(|v| Vec3::from(v)).collect();

    let mut skelett_lines: Vec<Vec3> = Vec::new();
    for segment in &bvh.roots[0].edges
    {
        println!("{}, {}", segment.iv1, segment.iv2);
        println!("{}, {}", skelett_vertices[segment.iv1], skelett_vertices[segment.iv2]);
        skelett_lines.push(Vec3::from(skelett_vertices[segment.iv1])/50.0);
        skelett_lines.push(Vec3::from(skelett_vertices[segment.iv2])/50.0);
    }
    
    let skelett = Box::new(Lines::new(&skelett_lines, shader_program));
    renderer.drawables.push(skelett);


    let mut camera = Camera::new();

    let mut clock = Clock::new();

    while !renderer.stop_requested()
    {
        let delta_time = clock.start_next_iteration();

        renderer.render(&camera);

        let dtheta = clock.get_total_time() * 2.0;

        camera.position = 50.0 * Vec3 { x: dtheta.sin(), y: 0.0, z: dtheta.cos() };

        renderer.flush_events();
    }

}

fn main() {
    // main_parser();
    main_renderer_test();
}
