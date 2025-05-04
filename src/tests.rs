// tests.rs
// Contains unit tests for verifying correctness of similarity computation and graph construction.

#[cfg(test)]
mod tests {
    use crate::similarity::cosine_similarity;
    use crate::models::Team;
    use crate::graph_utils::build_graph;

    /// Test cosine similarity between two identical vectors.
    /// Expect value to be 1.0 (perfect similarity).
    #[test]
    fn test_cosine_similarity_basic() {
        let a = vec![1.0, 0.0];
        let b = vec![1.0, 0.0];
        assert_eq!(cosine_similarity(&a, &b), 1.0);
    }

    /// Test cosine similarity between orthogonal vectors.
    /// Expect value to be 0.0 (no similarity).
    #[test]
    fn test_cosine_similarity_orthogonal() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        assert_eq!(cosine_similarity(&a, &b), 0.0);
    }

    /// Test that all teams are properly added to the graph.
    /// Expect node count to match number of teams.
    #[test]
    fn test_graph_building_node_count() {
        let teams = vec![
            Team { name: "A".to_string(), season: "2023".to_string(), stats: vec![1.0, 0.0] },
            Team { name: "B".to_string(), season: "2023".to_string(), stats: vec![0.0, 1.0] },
            Team { name: "C".to_string(), season: "2023".to_string(), stats: vec![1.0, 0.0] },
        ];
        let (graph, _) = build_graph(&teams, 0.5);
        assert_eq!(graph.node_count(), 3);
    }

    /// Test that an edge is formed between highly similar teams.
    /// With a threshold of 0.9, identical vectors should result in an edge.
    #[test]
    fn test_graph_edge_creation() {
        let teams = vec![
            Team { name: "A".to_string(), season: "2023".to_string(), stats: vec![1.0, 0.0] },
            Team { name: "B".to_string(), season: "2023".to_string(), stats: vec![1.0, 0.0] },
        ];
        let (graph, _) = build_graph(&teams, 0.9);
        assert_eq!(graph.edge_count(), 1);
    }
}
