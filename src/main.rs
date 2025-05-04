mod models;
mod io_utils;
mod similarity;
mod graph_utils;

#[cfg(test)]
mod tests;

use models::Team;
use io_utils::{read_csv, write_to_file};
use graph_utils::{build_graph, compute_closeness_centrality, densest_subgraph};

fn main() {
    let file_path = "DEV _ March Madness.csv";
    let teams = read_csv(file_path);

    let (graph, _node_map) = build_graph(&teams, 0.75);
    let closeness = compute_closeness_centrality(&graph);
    let (dense_subgraph, density) = densest_subgraph(&graph);

    let mut output = String::new();

    // Summary stats
    output.push_str(&format!("Graph has {} nodes and {} edges\n", graph.node_count(), graph.edge_count()));
    let avg_degree = if graph.node_count() > 0 {
        graph.edge_count() as f64 * 2.0 / graph.node_count() as f64
    } else {
        0.0
    };
    output.push_str(&format!("Average node degree: {:.2}\n", avg_degree));

    // Top 5 closeness centrality scores
    let mut top_centrality: Vec<_> = closeness.iter().collect();
    top_centrality.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    output.push_str("\nTop 5 teams by closeness centrality:\n");
    for (&node, score) in top_centrality.iter().copied().take(5) {
        output.push_str(&format!("{}: {:.3}\n", graph[node], score));
    }

    // Densest subgraph summary
    output.push_str(&format!(
        "\nDensest subgraph: {} nodes, density = {:.3}\n",
        dense_subgraph.node_count(),
        density
    ));

    // Top 5 nodes in densest subgraph by degree
    let mut degrees: Vec<_> = dense_subgraph.node_indices()
        .map(|n| (dense_subgraph[n].clone(), dense_subgraph.neighbors(n).count()))
        .collect();
    degrees.sort_by_key(|&(_, deg)| std::cmp::Reverse(deg));

    output.push_str("Top 5 nodes in densest subgraph by degree:\n");
    for (name, deg) in degrees.iter().take(5) {
        output.push_str(&format!("{} (degree {})\n", name, deg));
    }

    write_to_file("output_results.txt", &output);
    println!("Results written to output_results.txt");
}
