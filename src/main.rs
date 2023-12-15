mod graph;
mod reader;
mod algorithm;
mod tests;

use std::io;
use graph::{create_adjacency_list};
use reader::read_weighted_graph_from_file;
use algorithm::{analyze_by_degrees_of_separation, find_detailed_important_nodes_by_degree};

fn main() -> io::Result<()> {
    let weighted_graph = read_weighted_graph_from_file("San-Joaquin.txt")?;
    let adjacency_list = create_adjacency_list(&weighted_graph);
    let max_degree = 6;
    let degree_results = analyze_by_degrees_of_separation(&adjacency_list, max_degree);

    let important_nodes = find_detailed_important_nodes_by_degree(&degree_results);

    for (degree, node_info) in important_nodes.iter().enumerate() {
        match node_info {
            Some((important_node_index, detailed_results)) => {
                if detailed_results[degree].0 == 0 {
                    println!("No important node found for degree {}", degree + 1);
                } else {
                    println!("Most important node for degree {}: Node {}", degree + 1, important_node_index);
                    println!("Number of reachable nodes in degree {}: {}", degree + 1, detailed_results[degree].0);
                    println!("Average distance in degree {}: {:.2}", degree + 1, detailed_results[degree].1);
                    
                    println!("\nNumber of reachable nodes and average distances for Node {} for the other degrees:", important_node_index);
                    for (other_degree, &(reachable, avg_dist)) in detailed_results.iter().enumerate() {
                        if other_degree != degree {
                            println!("  - Degree {}: {} reachable nodes, Average Distance: {:.2}", other_degree + 1, reachable, avg_dist);
                        }
                    }
                    println!("-----------------------------------------------------------------------------------");
                }
            }
            None => println!("No important node found for degree {}", degree + 1),
        }
    }

    Ok(())
}


    
