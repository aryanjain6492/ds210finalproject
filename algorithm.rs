use super::graph::{AdjacencyList, Vertex, Distance};
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq)]
pub struct State {
    pub distance: Distance,
    pub vertex: Vertex,
    pub steps: usize,
}

// Implementing the Eq trait for State to support equality checks.
impl Eq for State {}

// Implementing PartialOrd to define a custom partial ordering for State instances.
// This is necessary because State contains a floating-point distance, which does not have total ordering.
impl PartialOrd for State {
    // Defines how two State instances are compared.
    // The comparison is based on the `distance` field of the State struct.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Compares `distance` of the other State with the current State's `distance`.
        // Uses `partial_cmp` since floating-point numbers (f64) implement PartialOrd.
        other.distance.partial_cmp(&self.distance)
    }
}

// Implementing Ord to provide a total ordering for State instances.
// This is required for State to be used in a BinaryHeap, as BinaryHeap needs to know how to order its elements.
impl Ord for State {
    // Provides a method to compare two State instances for ordering.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Unwraps the Option returned by partial_cmp. 
        // Since we know that distance will always be comparable (no NaN values), unwrap will not panic.
        other.partial_cmp(self).unwrap()
    }
}

pub fn analyze_by_degrees_of_separation(graph: &AdjacencyList, max_degree: usize) -> Vec<Vec<(usize, f64)>> {
    let mut results = vec![vec![]; max_degree];

    for start in 0..graph.len() {
        let mut distances: Vec<Option<Distance>> = vec![None; graph.len()];
        let mut degrees: Vec<Option<usize>> = vec![None; graph.len()];
        let mut pq = BinaryHeap::new();

        distances[start] = Some(0.0);
        degrees[start] = Some(0);
        pq.push(State { vertex: start, distance: 0.0, steps: 0 });

        while let Some(State { vertex: v, distance: dist, steps: s }) = pq.pop() {
            if s > max_degree {
                continue;
            }

            for &(neighbor, length) in &graph[v] {
                let next_dist = dist + length;
                let next_steps = s + 1;
                if next_steps <= max_degree && (distances[neighbor].is_none() || next_dist < distances[neighbor].unwrap()) {
                    distances[neighbor] = Some(next_dist);
                    degrees[neighbor] = Some(next_steps);
                    pq.push(State { vertex: neighbor, distance: next_dist, steps: next_steps });
                }
            }
        }

        // Aggregate results by degree
        for degree in 1..=max_degree {
            let nodes_at_degree = degrees.iter().filter(|&&d| d == Some(degree)).count();
            let total_distance: Distance = degrees.iter().enumerate()
                .filter(|&(_, &d)| d == Some(degree))
                .filter_map(|(idx, _)| distances[idx])
                .sum();
            let average_distance = if nodes_at_degree > 0 {
                total_distance / nodes_at_degree as f64
            } else {
                0.0
            };
            results[degree - 1].push((nodes_at_degree, average_distance));
        }
    }

    results
}

pub fn find_detailed_important_nodes_by_degree(results: &[Vec<(usize, f64)>]) -> Vec<Option<(usize, Vec<(usize, f64)>)>> {
    let mut detailed_important_nodes = Vec::new();

    for degree_results in results {
        let cutoff_index = (degree_results.len() as f64 * 0.1).ceil() as usize;
        let cutoff_index = std::cmp::max(1, cutoff_index);

        let mut sorted_results = degree_results.to_vec();
        sorted_results.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.partial_cmp(&b.1).unwrap()));
        let top_nodes = sorted_results.iter().take(cutoff_index);

        let most_important = top_nodes.min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|&(reachable, avg_dist)| {
                if reachable == 0 {
                    // If the node has 0 reachable nodes, return None
                    None
                } else {
                    // Otherwise, find the index and retrieve detailed information
                    let index = degree_results.iter().position(|&r| r == (reachable, avg_dist)).unwrap();
                    let detailed_info = results.iter().map(|res| res[index]).collect();
                    Some((index, detailed_info))
                }
            }).flatten();

        detailed_important_nodes.push(most_important);
    }

    detailed_important_nodes
}





