mod benchmark;
mod parsing;

use benchmark::run_test;
use serde_json;

use std::{collections::HashMap, fs};

// Config
const N_BENCHMARK_FILES: u32 = 3;
const N_TEST_EXECUTIONS: usize = 2;

fn main() {
    let single_target_output_file_path =
        String::from("./output/single_target_petgraph_benchmarks.json");
    let all_targets_output_file_path =
        String::from("./output/all_targets_petgraph_benchmarks.json");
    run_benchmark(true, single_target_output_file_path);
    run_benchmark(false, all_targets_output_file_path);
}

fn run_benchmark(single_target: bool, output_file_path: String) {
    println!("Starting {}", output_file_path);
    let input_files_paths = get_input_files_paths();

    let benchmarks: HashMap<&str, u128> = input_files_paths
        .iter()
        .map(|file_path| {
            (
                get_file_name(file_path),
                run_test(file_path, N_TEST_EXECUTIONS, single_target),
            )
        })
        .collect();

    let output_file_content = serde_json::to_string(&benchmarks).unwrap();

    fs::write(output_file_path, output_file_content)
        .expect("Should be able to write to output file.");
}

fn get_input_files_paths() -> Vec<String> {
    (0..N_BENCHMARK_FILES)
        .map(|index| format!("./input/benchmark_{}.in", index))
        .collect()
}

fn get_file_name(file_path: &String) -> &str {
    file_path.split("/").last().unwrap()
}
