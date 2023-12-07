use crate::graph::Graph;
use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn read_tsp_file(filename: &String) -> std::io::Result<Graph<usize>> {
    let file: File = File::open(filename)?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut vertices: Vec<usize> = Vec::new();
    let mut adjacency_matrix: Vec<Vec<u64>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let _ = line.replace("  ", "");

        let values: Vec<u64> = line.split_whitespace().map(|x| { x.parse::<u64>().unwrap() } ).collect();
        adjacency_matrix.push(values);
    }

    vertices = (0..adjacency_matrix.len()).collect();

    Ok(Graph::<usize>::from(vertices, adjacency_matrix))
}