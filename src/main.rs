// src/main.rs

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Write};
use csv::Reader;
use ndarray::{Array2, Array1};
use itertools::Itertools;
use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::prelude::*;
use std::f64;

#[derive(Debug, Clone)]
struct Team {
    name: String,
    season: String,
    stats: Vec<f64>,
}

fn read_csv(path: &str) -> Vec<Team> {
    let mut rdr = Reader::from_path(path).expect("CSV file not found");
    let headers = rdr.headers().unwrap().clone();
    let name_col = headers.iter().position(|h| h == "Full Team Name").expect("'Full Team Name' column not found");
    let season_col = headers.iter().position(|h| h == "Season").expect("'Season' column not found");
    let stat_cols: Vec<_> = headers.iter().enumerate()
        .filter(|&(i, h)| i != name_col && i != season_col && h != "Seed")
        .map(|(i, _)| i)
        .collect();

    let mut teams = Vec::new();
    for result in rdr.records() {
        let record = result.unwrap();
        let name = record.get(name_col).unwrap_or("").to_string();
        let season = record.get(season_col).unwrap_or("").to_string();
        let stats: Vec<f64> = stat_cols.iter()
            .map(|&i| record.get(i).unwrap_or("0").parse::<f64>().unwrap_or(0.0))
            .collect();

        if !name.is_empty() && stats.iter().any(|&x| x != 0.0) {
            teams.push(Team { name: format!("{} ({})", name, season), season, stats });
        }
    }
    teams
}

fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let dot: f64 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a = a.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    let norm_b = b.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}

fn build_graph(teams: &[Team], threshold: f64) -> (UnGraph<String, f64>, HashMap<String, NodeIndex>) {
    let mut graph = UnGraph::<String, f64>::new_undirected();
    let mut node_map = HashMap::new();

    for team in teams {
        let index = graph.add_node(team.name.clone());
        node_map.insert(team.name.clone(), index);
    }

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

fn densest_subgraph(graph: &UnGraph<String, f64>) -> (UnGraph<String, f64>, f64) {
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
        if let Some((node, _)) = subgraph.node_indices()
            .map(|n| (n, subgraph.edges(n).count()))
            .min_by_key(|&(_, deg)| deg) {
            subgraph.remove_node(node);
        }
    }

    (best_subgraph, best_density)
}

fn compute_closeness_centrality(graph: &UnGraph<String, f64>) -> HashMap<NodeIndex, f64> {
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
        if total_dist > 0.0 {
            result.insert(node, (n as f64 - 1.0) / total_dist);
        } else {
            result.insert(node, 0.0);
        }
    }
    result
}

fn write_to_file(path: &str, content: &str) {
    let mut file = File::create(path).expect("Unable to create file");
    file.write_all(content.as_bytes()).expect("Unable to write data");
}

fn main() {
    let file_path = "DEV _ March Madness.csv";
    let teams = read_csv(file_path);

    let (graph, _node_map) = build_graph(&teams, 0.75);
    let closeness = compute_closeness_centrality(&graph);
    let (dense_subgraph, density) = densest_subgraph(&graph);

    let mut output = String::new();

    output.push_str(&format!("Graph has {} nodes and {} edges\n", graph.node_count(), graph.edge_count()));
    output.push_str("\nNode degrees:\n");
    for node in graph.node_indices() {
        output.push_str(&format!("{} has {} neighbors\n", graph[node], graph.neighbors(node).count()));
    }

    output.push_str("\nTop closeness centrality scores:\n");
    for (node, &score) in &closeness {
        output.push_str(&format!("{}: {:.3}\n", graph[*node], score));
    }

    output.push_str(&format!("\nDensest subgraph has {} nodes, density = {:.3}\n", dense_subgraph.node_count(), density));

    output.push_str("\nTeams in the densest subgraph:\n");
    for node in dense_subgraph.node_indices() {
        output.push_str(&format!("{}\n", dense_subgraph[node]));
    }

    write_to_file("output_results.txt", &output);
    println!("Results written to output_results.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

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