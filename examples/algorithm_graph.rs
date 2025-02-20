use numberlab::algorithm::graph::{bfs, dfs, dijkstra};
use numberlab::structure::graph::Graph;

fn main() {
    let graph = &Graph::from_adjacency_matrix_with_labels(
        ["A", "B", "C", "D", "E", "F"],
        [
            [None, Some(6.5), None, None, None, Some(9.5)],
            [None, None, Some(1.5), None, None, None],
            [None, None, None, None, None, Some(2.5)],
            [None, None, Some(4.5), None, None, None],
            [None, None, None, Some(2.5), None, None],
            [None, None, None, None, Some(8.5), None],
        ],
    );

    println!("{}", graph);

    println!();

    println!("DFS {:?}\nBFS {:?}\n", dfs(graph, 0, 0), bfs(graph, 0, 0));
    println!("DFS {:?}\nBFS {:?}\n", dfs(graph, 0, 1), bfs(graph, 0, 1));
    println!("DFS {:?}\nBFS {:?}\n", dfs(graph, 0, 2), bfs(graph, 0, 2));
    println!("DFS {:?}\nBFS {:?}\n", dfs(graph, 0, 3), bfs(graph, 0, 3));
    println!("DFS {:?}\nBFS {:?}\n", dfs(graph, 0, 4), bfs(graph, 0, 4));
    println!("DFS {:?}\nBFS {:?}\n", dfs(graph, 0, 5), bfs(graph, 0, 5));

    println!(
        "{:?}",
        dijkstra(graph, 0, 3)
            .iter()
            .map(|p| format!("{}({})", p.1, p.2))
            .collect::<Vec<String>>()
            .join(" --> ")
    );
}
