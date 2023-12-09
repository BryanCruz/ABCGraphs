use std::{fs, time::Instant};

use petgraph::{algo::dijkstra, graph::NodeIndex, graph::UnGraph};

use crate::parsing::{parse_graph, parse_input_file};

pub fn run_test(file_path: &String, n_executions: usize) -> u128 {
    let file_content = fs::read_to_string(file_path).expect("Should be able to read file");
    let input_graph = parse_input_file(&file_content);
    let parsed_graph = parse_graph(&input_graph);

    let execution_times_in_ms: Vec<u128> = (0..n_executions)
        .map(|_| {
            let start_time = Instant::now();
            run_algorithm_for_all_targets(&parsed_graph, &input_graph.queries);
            let end_time = Instant::now();

            (end_time - start_time).as_millis()
        })
        .collect();

    let execution_times_in_ms_sum: u128 = execution_times_in_ms.iter().sum();
    let execution_time_in_ms = execution_times_in_ms_sum / (execution_times_in_ms.len() as u128);
    execution_time_in_ms
}

fn run_algorithm_for_single_target(graph: &UnGraph<u32, u32>, queries: &Vec<(u32, u32)>) {
    for (a, b) in queries {
        let node_a: NodeIndex = NodeIndex::new(*a as usize);
        let node_b: NodeIndex = NodeIndex::new(*b as usize);

        dijkstra(&graph, node_a, Some(node_b), |edge| *edge.weight());
    }
}

fn run_algorithm_for_all_targets(graph: &UnGraph<u32, u32>, queries: &Vec<(u32, u32)>) {
    for (a, b) in queries {
        let node_a: NodeIndex = NodeIndex::new(*a as usize);

        dijkstra(&graph, node_a, None, |edge| *edge.weight());
    }
}
