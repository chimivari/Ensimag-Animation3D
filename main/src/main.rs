use bvh_parser::prelude::*;

fn main() {
    let bvh_file = std::fs::read_to_string("inputs/walk1.bvh").unwrap();
    parse(bvh_file);
}
