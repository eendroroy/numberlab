use numberlab::structure::graph::{bfs, dfs, Graph};

fn main() {
    let graph = &Graph::from(
        ["A", "B", "C", "D", "E", "F"],
        [
            [None, Some(1), None, None, None, Some(2)],
            [None, None, Some(3), None, None, None],
            [None, None, None, None, None, Some(4)],
            [None, None, Some(5), None, None, None],
            [None, None, None, Some(6), None, None],
            [None, None, None, None, Some(7), None],
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
