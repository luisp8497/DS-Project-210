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

    output.push_str(&format!(
        "Graph has {} nodes and {} edges\n",
        graph.node_count(),
        graph.edge_count()
    ));

    output.push_str("\nNode degrees:\n");
    for node in graph.node_indices() {
        output.push_str(&format!(
            "{} has {} neighbors\n",
            graph[node],
            graph.neighbors(node).count()
        ));
    }

    output.push_str("\nTop closeness centrality scores:\n");
    for (node, &score) in &closeness {
        output.push_str(&format!("{}: {:.3}\n", graph[*node], score));
    }

    output.push_str(&format!(
        "\nDensest subgraph has {} nodes, density = {:.3}\n",
        dense_subgraph.node_count(),
        density
    ));

    output.push_str("\nTeams in the densest subgraph:\n");
    for node in dense_subgraph.node_indices() {
        output.push_str(&format!("{}\n", dense_subgraph[node]));
    }

    write_to_file("output_results.txt", &output);
    println!("Results written to output_results.txt");
}
