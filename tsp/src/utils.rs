use crate::graph::Graph;
use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn read_tsp_file(filename: String) -> std::io::Result<Graph<i32>> {
    let file: File = File::open(filename)?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut adjacency_matrix: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let _ = line.replace("  ", "");

        let values: Vec<i32> = line.split_whitespace().map(|x| { x.parse::<i32>().unwrap() } ).collect();
        adjacency_matrix.push(values);
    }

    Ok(Graph::<i32>::from(adjacency_matrix))
}