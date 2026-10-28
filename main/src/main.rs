use bvh_parser::prelude::*;
use ::renderer::*;

#[warn(unused)]
fn main_parser()
{
    let bvh_file = std::fs::read_to_string("inputs/Example1.bvh").unwrap();
    let bvh = parse(bvh_file).unwrap();
    let _ = bvh.roots[0].save("output2.txt");
}

fn main() {
    main_parser();
    main_renderer_test();
}
