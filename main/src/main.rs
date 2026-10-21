use bvh_parser::prelude::*;

fn main() {
    let bvh_file = std::fs::read_to_string("inputs/Example1.bvh").unwrap();
    let bvh = parse(bvh_file);
    if let Err(err) = bvh {
        println!("{err:?}");
    }
}
