use bvh_parser::prelude::*;

fn main() {
    let bvh_file = std::fs::read_to_string("inputs/Example1.bvh").unwrap();
    let bvh = parse(bvh_file).unwrap();
    let _ = bvh.roots[0].save("output2.txt");
}
