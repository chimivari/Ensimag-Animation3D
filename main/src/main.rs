use core::panic;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use engine::clock::Clock;
use engine::core::world::Entity;
use engine::prelude::*;
use engine::rendering::camera::Camera;
use engine::resource::meshloader::mesh_from_off_string;
// use engine::resource::meshloader::mesh_from_off_string;
use glam::Vec3;

use engine::rendering::drawable::Lines;
use engine::rendering::renderer::{self, Renderer};
use engine::rendering::shader::{ShaderConfig, ShaderSourceType};

// #[warn(unused)]
// fn main_renderer_test() {
//     let vertex_shader_path = "./resources/shaders/base.vert";
//     let fragment_shader_path = "./resources/shaders/base.frag";
//
//     let mut renderer = Renderer::try_new(500, 300, "Kinematic animations").unwrap();
//
//     let vertex_shader = ShaderConfig {
//         name: "default vertex shader".to_string(),
//         source_type: ShaderSourceType::FileName(vertex_shader_path.to_string()),
//     };
//
//     let fragment_shader = ShaderConfig {
//         name: "default fragment shader".to_string(),
//         source_type: ShaderSourceType::FileName(fragment_shader_path.to_string()),
//     };
//
//     let shader_program = renderer
//         .create_shader_program(vertex_shader, fragment_shader)
//         .unwrap();
//
//     // Crash in case of IOError
//     // let skin = mesh_from_off_string(std::fs::read_to_string("./resources/skin.off").unwrap());
//
//     let bvh_file = std::fs::read_to_string("inputs/walk1.bvh").unwrap();
//     let bvh = parse(bvh_file).unwrap();
//
//     // Contains lines for each animation frame
//     let mut skelett_lines: Vec<Vec<Vec3>> = Vec::new();
//     for frame in &bvh.roots[0].frames {
//         let mut lines: Vec<Vec3> = Vec::new();
//         for segment in &bvh.roots[0].edges {
//             lines.push(Vec3::from(frame.vertices[segment.iv1]) / 50.0);
//             lines.push(Vec3::from(frame.vertices[segment.iv2]) / 50.0);
//         }
//         skelett_lines.push(lines);
//     }
//
//     let mut skelett = Rc::new(RefCell::new(Lines::new(&skelett_lines[1], shader_program)));
//     renderer.drawables.push(skelett.clone());
//
//     let mut camera = Camera::new();
//
//     let mut clock = Clock::new();
//
//     let camera_distance = 30.0;
//
//     let animation_frame_delta = 1.0 / 24.0;
//     let mut next_frame_time = clock.get_total_time() + animation_frame_delta;
//     let mut frame_index = 0;
//
//     while !renderer.stop_requested() {
//         let _delta_time = clock.start_next_iteration();
//         let current_time = clock.get_total_time();
//
//         // Rotate camera over time
//         let dtheta = current_time * 2.0;
//         camera.position = camera_distance
//             * Vec3 {
//                 x: dtheta.sin(),
//                 y: 0.5,
//                 z: dtheta.cos(),
//             };
//
//         if current_time > next_frame_time {
//             next_frame_time = current_time + animation_frame_delta;
//             // The first frame isn't an actual frame but just the default pose.
//             //  => there are n-1 frames, and indices start at 1
//             frame_index = ((frame_index + 1) % (skelett_lines.len() - 1)) + 1;
//
//             skelett
//                 .borrow_mut()
//                 .update_all_vertices(&skelett_lines[frame_index]);
//         }
//
//         renderer.render(&camera);
//
//         renderer.flush_events();
//     }
// }

fn main() {
    // main_parser();
    // main_renderer_test();
    let WIDTH = 600;
    let HEIGHT = 500;
    

    let bvh_file = std::fs::read_to_string("inputs/Example1.bvh").unwrap();
    let mut bvh = parse(bvh_file).unwrap();

    let mut world = World::new(bvh.frame_time() as f64);

    // Spawn 1 entity per joint
    let joints = bvh.generate_components();
    let entities = joints.iter().map(|_| world.spawn()).collect::<Vec<_>>();

    // Add components to each entity
    joints
        .into_iter()
        .zip(entities.iter())
        .for_each(|((joint, joint_records), &entity)| {
            let Joint {
                parent,
                children,
                transform,
            } = joint;

            let parent = parent.map(|i| entities[i]);
            let children = children
                .into_iter()
                .map(|i| entities[i])
                .collect::<Vec<_>>();

            let joint = Joint::new_with_children(parent, transform, children);
            world.add_components_to(entity, (joint, joint_records));
        });

    // Create clock
    let clock = Clock::new();
    let clock_ent = world.spawn();
    world.add_components_to(clock_ent, clock);
    

    // Create renderer
    let mut renderer = Renderer::try_new(WIDTH, HEIGHT, "Kinematic animations").unwrap();

    let vertex_shader_path = "./resources/shaders/base.vert";
    let fragment_shader_path = "./resources/shaders/base.frag";


    let vertex_shader = ShaderConfig {
        name: "default vertex shader".to_string(),
        source_type: ShaderSourceType::FileName(vertex_shader_path.to_string()),
    };

    let fragment_shader = ShaderConfig {
        name: "default fragment shader".to_string(),
        source_type: ShaderSourceType::FileName(fragment_shader_path.to_string()),
    };

    let shader_program = renderer
        .create_shader_program(vertex_shader, fragment_shader)
        .unwrap();

    let mesh = mesh_from_off_string(&std::fs::read_to_string("./resources/skin.off").unwrap(), shader_program).unwrap();
    let body = world.spawn();
    world.add_components_to(body, mesh);

    let renderer_ent = world.spawn();
    world.add_components_to(renderer_ent, renderer);

    // Create camera
    let camera = Camera::new();
    let camera_ent = world.spawn();
    world.add_components_to(camera_ent, camera);

    world
        .add_system(Scheduler::Startup, start_renderer)
        .add_system(Scheduler::Update, render)
        .add_system(Scheduler::Update, update)
        .run();
}

fn update(world: &mut World) {

    // Poll events
    let mut should_stop = false;
    for (&e, renderer) in world.iter_component_mut::<Renderer>().unwrap()
    {
        renderer.flush_events();
        should_stop = renderer.stop_requested();
    }
    if should_stop { world.stop(); }

    let mut current_time = 0.0;
    let mut _delta_time = 0.0;

    // Get clock time and delta time
    if let Some(clocks) = world.iter_component_mut::<Clock>()
    {
        for (_, clock) in clocks
        {
            _delta_time = clock.start_next_iteration();
            current_time = clock.get_total_time();
        }
    }

    if let Some(cameras) = world.iter_component_mut::<Camera>()
    {
        for (_, camera) in cameras
        {
            let dtheta = current_time * 2.0;
            camera.position = 200.0 * Vec3 {
                    x: dtheta.sin(),
                    y: 0.5,
                    z: dtheta.cos(),
                };
        }
    }

    // let mut transforms = VecDeque::new();
    //
    // for (&e, records) in world.iter_component_mut::<JointRecordsQueue>().unwrap() {
    //     if let Some(record) = records.pop() {
    //         transforms.push_back((e, record));
    //     } else if !transforms.is_empty() {
    //         panic!("Every joint should have the same number of records");
    //     }
    // }
    //
    // for (&e1, joint) in world.iter_component_mut::<Joint<Entity>>().unwrap() {
    //     let (e2, record) = transforms.pop_front().unwrap();
    //     assert_eq!(e1, e2);
    //     joint.transform = record.transform;
    // }
    //
    // assert!(transforms.is_empty());
}


fn render(world: &mut World)
{
    for (&e, renderer) in world.iter_component::<Renderer>().unwrap()
    {
        renderer.render(&world);
    }

    for (&e, renderer) in world.iter_component_mut::<Renderer>().unwrap()
    {
        renderer.swap_buffers();
    }
}

fn start_renderer(world: &mut World)
{
    for (&e, renderer) in world.iter_component::<Renderer>().unwrap()
    {
        renderer.start();
    }
}
