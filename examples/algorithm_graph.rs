use numberlab::algorithm::graph::{bfs, dfs};
use numberlab::structure::graph::Graph;

fn main() {
    let graph = &Graph::from(
        ["A", "B", "C", "D", "E", "F"],
        [
            [None, Some(6.3), None, None, None, Some(9.8)],
            [None, None, Some(1.2), None, None, None],
            [None, None, None, None, None, Some(2.7)],
            [None, None, Some(4.5), None, None, None],
            [None, None, None, Some(2.1), None, None],
            [None, None, None, None, Some(8.6), None],
        ],
    );

    println!("{}", graph);

    println!();

    println!("DFS - {:?}, BFS - {:?}", dfs(graph, 0, 0), bfs(graph, 0, 0));
    println!("DFS - {:?}, BFS - {:?}", dfs(graph, 0, 1), bfs(graph, 0, 1));
    println!("DFS - {:?}, BFS - {:?}", dfs(graph, 0, 2), bfs(graph, 0, 2));
    println!("DFS - {:?}, BFS - {:?}", dfs(graph, 0, 3), bfs(graph, 0, 3));
    println!("DFS - {:?}, BFS - {:?}", dfs(graph, 0, 4), bfs(graph, 0, 4));
    println!("DFS - {:?}, BFS - {:?}", dfs(graph, 0, 5), bfs(graph, 0, 5));
}
