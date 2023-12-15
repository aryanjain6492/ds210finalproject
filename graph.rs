pub type Vertex = usize;
pub type Distance = f64;
pub type WeightedEdge = (Vertex, Vertex, Distance);

pub struct WeightedGraph {
    pub edges: Vec<WeightedEdge>,
    pub n: usize,
}

pub type AdjacencyList = Vec<Vec<(Vertex, Distance)>>;

pub fn create_adjacency_list(graph: &WeightedGraph) -> AdjacencyList {
    let mut adjacency_list: AdjacencyList = vec![Vec::new(); graph.n];

    for &(u, v, length) in &graph.edges {
        adjacency_list[u].push((v, length));
    }

    adjacency_list
} 