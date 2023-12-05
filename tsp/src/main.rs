use std::env;
use std::process;
use crate::graph::Graph;

pub mod utils;
pub mod graph;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <filename> <algorithm>", args[0]);
        println!("    <filename> - TSP file to read");
        println!("    <algorithm> - Algorithm to use (exact, 2opt)");
        process::exit(1);
    }

    let filename = args.remove(1);
    let algorithm = args.remove(1);

    let t: Graph<usize> = utils::read_tsp_file(&filename).unwrap();

    let _results: (Vec<usize>, u64);

    if algorithm == "exact" {
        _results = match t.tsp_exact(true) {
            Ok(r) => r,
            Err(e) => {
                println!("Error: {}", e);
                process::exit(1);
            }
        };
    } else if algorithm == "2opt" {
        _results = match t.tsp_2_opt_approx(true) {
            Ok(r) => r,
            Err(e) => {
                println!("Error: {}", e);
                process::exit(1);
            }
        };
    } else {
        println!("Error: Unknown algorithm {}", algorithm);
        process::exit(1);
    }
}