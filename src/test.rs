#[cfg(test)]
mod tests {
    use crate::similarity::cosine_similarity;
    use crate::models::Team;
    use crate::graph_utils::build_graph;

    #[test]
    fn test_cosine_similarity_basic() {
        let a = vec![1.0, 0.0];
        let b = vec![1.0, 0.0];
        assert_eq!(cosine_similarity(&a, &b), 1.0);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        assert_eq!(cosine_similarity(&a, &b), 0.0);
    }

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
