use petgraph::graph::UnGraph;

const N_BENCHMARK_FILES: u32 = 1;

#[derive(Debug)]
pub struct InputGraph {
    pub n_vertices: u32,
    pub n_edges: u32,
    pub edges: Vec<(u32, u32, u32)>,
    pub queries: Vec<(u32, u32)>,
}

pub fn get_input_files_paths() -> Vec<String> {
    (0..N_BENCHMARK_FILES)
        .map(|index| {
            format!(
                "/home/bryan/documents/faculdade/pgc/sage/files/input/benchmark_{}.in",
                index
            )
        })
        .collect()
}

pub fn get_output_file_path() -> String {
    String::from("/home/bryan/documents/faculdade/pgc/sage/files/output/rust_benchmarks.json")
}

pub fn parse_graph(input_graph: &InputGraph) -> UnGraph<u32, u32> {
    let graph: UnGraph<u32, u32> = UnGraph::from_edges(&input_graph.edges);
    graph
}

pub fn parse_input_file(contents: &str) -> InputGraph {
    let lines = parse_lines(contents);

    let (n_vertices, n_edges) = parse_tuple(lines.get(0).unwrap());

    let edges_lines = &lines[1..((n_edges + 1) as usize)];
    let mut edges = Vec::new();
    for line in edges_lines {
        edges.push(parse_triple(line));
    }

    let queries_lines = &lines[((n_edges + 1) as usize)..];
    let mut queries = Vec::new();
    for line in queries_lines {
        queries.push(parse_tuple(line));
    }

    InputGraph {
        n_vertices,
        n_edges,
        edges,
        queries,
    }
}

fn parse_lines(contents: &str) -> Vec<Vec<u32>> {
    let mut lines = Vec::new();
    for line in contents.lines() {
        lines.push(parse_line(&line.to_string()));
    }
    lines
}

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn parse_tuple(vec: &Vec<u32>) -> (u32, u32) {
    (vec[0], vec[1])
}

fn parse_triple(vec: &Vec<u32>) -> (u32, u32, u32) {
    (vec[0], vec[1], vec[2])
}
