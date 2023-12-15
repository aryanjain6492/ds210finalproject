use super::graph::{WeightedEdge, WeightedGraph, Vertex, Distance};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_weighted_graph_from_file(file_path: &str) -> io::Result<WeightedGraph> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut edges: Vec<WeightedEdge> = Vec::new();
    let mut max_vertex = 0;

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 4 {
            let start_node = parts[1].parse::<Vertex>().unwrap();
            let end_node = parts[2].parse::<Vertex>().unwrap();
            let weight = parts[3].parse::<Distance>().unwrap();

            edges.push((start_node, end_node, weight));
            edges.push((end_node, start_node, weight));
            max_vertex = max_vertex.max(start_node).max(end_node);
        }
    }
    let n = max_vertex + 1; 
    Ok(WeightedGraph { edges, n })
}
