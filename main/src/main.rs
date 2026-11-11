use std::rc::Rc;
use std::cell::RefCell;

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
    
    let mut renderer = Renderer::try_new(500, 300, "Kinematic animations").unwrap();
    
    let vertex_shader = ShaderConfig {
        name: "default vertex shader".to_string(),
        source_type: ShaderSourceType::FileName(vertex_shader_path.to_string())
    };

    let fragment_shader = ShaderConfig {
        name: "default fragment shader".to_string(),
        source_type: ShaderSourceType::FileName(fragment_shader_path.to_string())
    };

    let shader_program = renderer.create_shader_program(vertex_shader, fragment_shader).unwrap();

    let bvh_file = std::fs::read_to_string("inputs/walk1.bvh").unwrap();
    let bvh = parse(bvh_file).unwrap();
    
    // Contains lines for each animation frame
    let mut skelett_lines: Vec<Vec<Vec3>> = Vec::new();
    for frame in &bvh.roots[0].frames{
        let mut lines: Vec<Vec3> = Vec::new();
        for segment in &bvh.roots[0].edges
        {
            lines.push(Vec3::from(frame.vertices[segment.iv1])/50.0);
            lines.push(Vec3::from(frame.vertices[segment.iv2])/50.0);
        }
        skelett_lines.push(lines);
    }
    
    let mut skelett = Rc::new(RefCell::new(Lines::new(&skelett_lines[1], shader_program)));
    renderer.drawables.push(skelett.clone());


    let mut camera = Camera::new();

    let mut clock = Clock::new();

    let camera_distance = 30.0;

    let animation_frame_delta = 1.0/24.0;
    let mut next_frame_time = clock.get_total_time() + animation_frame_delta;
    let mut frame_index = 0;

    while !renderer.stop_requested()
    {
        let _delta_time = clock.start_next_iteration();
        let current_time = clock.get_total_time();

        // Rotate camera over time
        let dtheta = current_time * 2.0;
        camera.position = camera_distance * Vec3 { x: dtheta.sin(), y: 0.5, z: dtheta.cos() };

        if current_time > next_frame_time
        {
            println!("{}", frame_index);
            next_frame_time = current_time + animation_frame_delta;
            // The first frame isn't an actual frame but just the default pose.
            //  => there are n-1 frames, and indices start at 1
            frame_index = ((frame_index + 1) % (skelett_lines.len() - 1)) + 1;

            skelett.borrow_mut().update_all_vertices(
                &skelett_lines[frame_index]
                );
        }


        renderer.render(&camera);

        renderer.flush_events();
    }

}

fn main() {
    // main_parser();
    main_renderer_test();
}
