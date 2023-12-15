#[cfg(test)]
mod tests {
    use crate::algorithm::{analyze_by_degrees_of_separation, find_detailed_important_nodes_by_degree};
    use crate::graph::{AdjacencyList};

    #[test]
    fn test_analyze_by_degrees_of_separation() {
        // Construct the adjacency list from the provided dataset
        let mut adjacency_list: AdjacencyList = vec![Vec::new(); 7]; // 7 nodes (0 to 6)
        adjacency_list[0] = vec![(1, 1.0), (2, 1.0), (3, 1.0), (4, 1.0), (5, 1.0), (6, 1.0)];
        adjacency_list[1] = vec![(2, 5.0)];
        adjacency_list[2] = vec![(3, 5.0), (4, 5.0)];
        adjacency_list[3] = vec![(4, 5.0), (5, 5.0)];
        adjacency_list[4] = vec![(5, 5.0), (6, 5.0)];
        adjacency_list[5] = vec![(6, 5.0)];

        // Add reverse edges for undirected graph
        for i in 0..adjacency_list.len() {
            let current_edges = adjacency_list[i].clone();
            for &(j, w) in &current_edges {
                adjacency_list[j].push((i, w));
            }
        }

        // Run the analysis for 6 degrees of separation
        let results = analyze_by_degrees_of_separation(&adjacency_list, 6);

        // Expected results
        // For each node, list the number of nodes reached and average distance for each degree
        let expected_results = vec![
            vec![(6, 1.0), (1, 1.0), (1, 1.0), (1, 1.0), (1, 1.0), (1, 1.0), (1, 1.0)], // Degree 1 for all nodes
            vec![(0, 0.0), (5, 2.0), (5, 2.0), (5, 2.0), (5, 2.0), (5, 2.0), (5, 2.0)], // Degree 2 for all nodes
            vec![(0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0)], // Degree 3 for all nodes
            vec![(0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0)], // Degree 4 for all nodes
            vec![(0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0)], // Degree 5 for all nodes
            vec![(0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0)], // Degree 6 for all nodes
        ];

        // Assert that results match expectations
        assert_eq!(results, expected_results);
    }

    #[test]
    fn test_find_detailed_important_nodes_by_degree() {
        // Mock results from `analyze_by_degrees_of_separation`
        let mock_results = vec![
            vec![(6, 1.0), (1, 1.0), (1, 1.0), (1, 1.0), (1, 1.0), (1, 1.0), (1, 1.0)], // Degree 1 for all nodes
            vec![(0, 0.0), (5, 2.0), (5, 2.0), (5, 2.0), (5, 2.0), (5, 2.0), (5, 2.0)], // Degree 2 for all nodes
            vec![(0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0)], // Degree 3 for all nodes
            vec![(0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0)], // Degree 4 for all nodes
            vec![(0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0)], // Degree 5 for all nodes
            vec![(0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0)], // Degree 6 for all nodes
        ];

        // Expected results
        let expected = vec![
            Some((0, vec![(6, 1.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0)])), // Most important node for degree 1
            Some((1, vec![(1, 1.0), (5, 2.0), (0, 0.0), (0, 0.0), (0, 0.0), (0, 0.0)])), // Most important node for degree 2
            None, // No important node for degree 3
            None, // No important node for degree 4
            None, // No important node for degree 5
            None, // No important node for degree 6
        ];        

        // Run the function
        let important_nodes = find_detailed_important_nodes_by_degree(&mock_results);

        // Assert that results match expectations
        assert_eq!(important_nodes, expected);
    }
}