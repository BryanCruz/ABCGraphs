mod benchmark;
mod parsing;

use benchmark::run_test;
use parsing::{get_input_files_paths, get_output_file_path};
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::dot::{Config, Dot};
use petgraph::graph::UnGraph;
use serde_json;

use std::{collections::HashMap, fs};

fn main() {
    let graph = UnGraph::<i32, i32>::from_edges(&[
        (1, 2, 10),
        (2, 3, 11),
        (3, 4, 10),
        (2, 4, 13),
        (1, 4, 15),
    ]);

    // Get the minimum spanning tree of the graph as a new graph, and check that
    // one edge was trimmed.
    let minimum_spanning_tree = UnGraph::<_, _>::from_elements(min_spanning_tree(&graph));
    assert_eq!(
        graph.raw_edges().len() - 2,
        minimum_spanning_tree.raw_edges().len()
    );

    // Output the tree to `graphviz` `DOT` format
    println!("{:?}", Dot::with_config(&graph, &[]));
    println!("{:?}", Dot::with_config(&minimum_spanning_tree, &[]));
}

fn benchmark_rust() {
    let n_test_executions = 1;
    let input_files_paths = get_input_files_paths();

    let execution_times_in_ms: HashMap<&str, u128> = input_files_paths
        .iter()
        .map(|file_path| {
            (
                get_file_name(file_path),
                run_test(file_path, n_test_executions),
            )
        })
        .collect();

    let output_file_path = get_output_file_path();
    let output_file_content = serde_json::to_string(&execution_times_in_ms).unwrap();

    fs::write(output_file_path, output_file_content)
        .expect("Should be able to write to output file.");
}

fn get_file_name(file_path: &String) -> &str {
    file_path.split("/").last().unwrap()
}
