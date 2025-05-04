// graph_utils.rs
// Constructs a graph from team similarities and provides analysis utilities like closeness and dense subgraph extraction.

use std::collections::{HashMap, HashSet};
use petgraph::graph::{UnGraph, NodeIndex};
use itertools::Itertools;
use crate::models::Team;
use crate::similarity::cosine_similarity;

/// Builds an undirected graph from a list of teams, connecting pairs with similarity >= threshold.
/// Outputs both the graph and a node index lookup map.
pub fn build_graph(teams: &[Team], threshold: f64) -> (UnGraph<String, f64>, HashMap<String, NodeIndex>) {
    let mut graph = UnGraph::<String, f64>::new_undirected();
    let mut node_map = HashMap::new();

    // Add each team as a node
    for team in teams {
        let index = graph.add_node(team.name.clone());
        node_map.insert(team.name.clone(), index);
    }

    // Add edges for pairs with sufficient similarity
    for (a, b) in teams.iter().tuple_combinations() {
        let sim = cosine_similarity(&a.stats, &b.stats);
        if sim >= threshold {
            let i = node_map[&a.name];
            let j = node_map[&b.name];
            graph.add_edge(i, j, sim);
        }
    }

    (graph, node_map)
}

/// Computes closeness centrality for each node in the graph.
/// Returns a map from NodeIndex to centrality score.
/// Uses BFS-style traversal to compute distance to all other nodes.
pub fn compute_closeness_centrality(graph: &UnGraph<String, f64>) -> HashMap<NodeIndex, f64> {
    let mut result = HashMap::new();
    for node in graph.node_indices() {
        let mut visited = HashSet::new();
        let mut queue = vec![(node, 0)];
        let mut total_dist = 0.0;

        while let Some((curr, dist)) = queue.pop() {
            if visited.insert(curr) {
                total_dist += dist as f64;
                for neighbor in graph.neighbors(curr) {
                    if !visited.contains(&neighbor) {
                        queue.push((neighbor, dist + 1));
                    }
                }
            }
        }

        let n = visited.len();
        result.insert(node, if total_dist > 0.0 {
            (n as f64 - 1.0) / total_dist
        } else {
            0.0
        });
    }

    result
}

/// Identifies the subgraph with the highest density (edges/nodes).
/// Uses greedy removal of lowest-degree nodes until density peaks.
pub fn densest_subgraph(graph: &UnGraph<String, f64>) -> (UnGraph<String, f64>, f64) {
    let mut subgraph = graph.clone();
    let mut best_density = 0.0;
    let mut best_subgraph = subgraph.clone();

    while subgraph.node_count() > 0 {
        let edges = subgraph.edge_count() as f64;
        let nodes = subgraph.node_count() as f64;
        let density = edges / nodes;
        if density > best_density {
            best_density = density;
            best_subgraph = subgraph.clone();
        }

        // Greedily remove the node with smallest degree
        if let Some((node, _)) = subgraph.node_indices()
            .map(|n| (n, subgraph.edges(n).count()))
            .min_by_key(|&(_, deg)| deg) {
            subgraph.remove_node(node);
        }
    }

    (best_subgraph, best_density)
}
