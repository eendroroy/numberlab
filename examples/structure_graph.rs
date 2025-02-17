use numberlab::structure::graph::{bfs, dfs, Graph};

fn main() {
    let graph = Graph::new_with_edges([
        [None, Some(1.0), None, None, None, None],
        [None, None, Some(1.0), None, None, Some(1.0)],
        [None, None, None, None, None, Some(1.0)],
        [None, None, Some(1.0), None, None, None],
        [None, None, None, None, None, Some(1.0)],
        [None, None, None, None, Some(1.0), None],
    ]);

    println!("{:?}", graph);
    println!("DFS - {:?}, BFS - {:?}", dfs(graph, 0, 0), bfs(graph, 0, 0));
    println!("DFS - {:?}, BFS - {:?}", dfs(graph, 0, 1), bfs(graph, 0, 1));
    println!("DFS - {:?}, BFS - {:?}", dfs(graph, 0, 2), bfs(graph, 0, 2));
    println!("DFS - {:?}, BFS - {:?}", dfs(graph, 0, 3), bfs(graph, 0, 3));
    println!("DFS - {:?}, BFS - {:?}", dfs(graph, 0, 4), bfs(graph, 0, 4));
    println!("DFS - {:?}, BFS - {:?}", dfs(graph, 0, 5), bfs(graph, 0, 5));
}
