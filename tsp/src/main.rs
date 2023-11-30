use std::env;

pub mod utils;
pub mod graph;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let filename = args.remove(1);

    let t = utils::read_tsp_file(filename).unwrap();
    println!("{:?}", t.get_adjacency_matrix());
}