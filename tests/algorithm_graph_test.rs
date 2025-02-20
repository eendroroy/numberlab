use numberlab::algorithm::graph::{bfs, dfs, dijkstra};
use numberlab::structure::graph::Graph;

#[test]
#[should_panic(expected = "Invalid start(6) or end(0) node, available nodes (0 - 5)")]
fn should_panic_if_source_invalid_for_dfs() {
    let graph = &Graph::<f32, 6>::new();
    dfs(graph, 6, 0);
}

#[test]
#[should_panic(expected = "Invalid start(0) or end(6) node, available nodes (0 - 5)")]
fn should_panic_if_destination_invalid_for_dfs() {
    let graph = &Graph::<f32, 6>::new();
    dfs(graph, 0, 6);
}

#[test]
#[should_panic(expected = "Invalid start(6) or end(0) node, available nodes (0 - 5)")]
fn should_panic_if_source_invalid_for_bfs() {
    let graph = &Graph::<f32, 6>::new();
    bfs(graph, 6, 0);
}

#[test]
#[should_panic(expected = "Invalid start(0) or end(6) node, available nodes (0 - 5)")]
fn should_panic_if_destination_invalid_for_bfs() {
    let graph = &Graph::<f32, 6>::new();
    bfs(graph, 0, 6);
}

#[test]
fn should_find_path_from_source_to_destination_using_dfs() {
    let graph = &Graph::from_adjacency_matrix([
        [None, Some(1.0), None, None, None, None],
        [None, None, Some(1.0), None, None, Some(1.0)],
        [None, None, None, None, None, Some(1.0)],
        [None, None, Some(1.0), None, None, None],
        [None, None, None, None, None, Some(1.0)],
        [None, None, None, None, Some(1.0), None],
    ]);

    assert_eq!(
        dfs(graph, 0, 0).iter().map(|p| p.0).collect::<Vec<usize>>(),
        vec![0]
    );
    assert_eq!(
        dfs(graph, 0, 1).iter().map(|p| p.0).collect::<Vec<usize>>(),
        vec![0, 1]
    );
    assert_eq!(
        dfs(graph, 0, 2).iter().map(|p| p.0).collect::<Vec<usize>>(),
        vec![0, 1, 2]
    );
    assert_eq!(
        dfs(graph, 0, 3).iter().map(|p| p.0).collect::<Vec<usize>>(),
        vec![]
    );
    assert_eq!(
        dfs(graph, 0, 4).iter().map(|p| p.0).collect::<Vec<usize>>(),
        vec![0, 1, 2, 5, 4]
    );
    assert_eq!(
        dfs(graph, 0, 5).iter().map(|p| p.0).collect::<Vec<usize>>(),
        vec![0, 1, 2, 5]
    );

    assert_eq!(
        dfs(graph, 0, 0)
            .iter()
            .map(|p| p.1.clone())
            .collect::<Vec<String>>(),
        vec!["0"]
    );
    assert_eq!(
        dfs(graph, 0, 1)
            .iter()
            .map(|p| p.1.clone())
            .collect::<Vec<String>>(),
        vec!["0", "1"]
    );
    assert_eq!(
        dfs(graph, 0, 2)
            .iter()
            .map(|p| p.1.clone())
            .collect::<Vec<String>>(),
        vec!["0", "1", "2"]
    );
    assert_eq!(
        dfs(graph, 0, 3)
            .iter()
            .map(|p| p.1.clone())
            .collect::<Vec<String>>(),
        Vec::<String>::new()
    );
    assert_eq!(
        dfs(graph, 0, 4)
            .iter()
            .map(|p| p.1.clone())
            .collect::<Vec<String>>(),
        vec!["0", "1", "2", "5", "4"]
    );
    assert_eq!(
        dfs(graph, 0, 5)
            .iter()
            .map(|p| p.1.clone())
            .collect::<Vec<String>>(),
        vec!["0", "1", "2", "5"]
    );
}

#[test]
fn should_find_path_from_source_to_destination_using_bfs() {
    let graph = &Graph::from_adjacency_matrix([
        [None, Some(1.0), None, None, None, None],
        [None, None, Some(1.0), None, None, Some(1.0)],
        [None, None, None, None, None, Some(1.0)],
        [None, None, Some(1.0), None, None, None],
        [None, None, None, None, None, Some(1.0)],
        [None, None, None, None, Some(1.0), None],
    ]);

    assert_eq!(
        bfs(graph, 0, 0).iter().map(|p| p.0).collect::<Vec<usize>>(),
        vec![0]
    );
    assert_eq!(
        bfs(graph, 0, 1).iter().map(|p| p.0).collect::<Vec<usize>>(),
        vec![0, 1]
    );
    assert_eq!(
        bfs(graph, 0, 2).iter().map(|p| p.0).collect::<Vec<usize>>(),
        vec![0, 1, 2]
    );
    assert_eq!(
        bfs(graph, 0, 3).iter().map(|p| p.0).collect::<Vec<usize>>(),
        vec![]
    );
    assert_eq!(
        bfs(graph, 0, 4).iter().map(|p| p.0).collect::<Vec<usize>>(),
        vec![0, 1, 5, 4]
    );
    assert_eq!(
        bfs(graph, 0, 5).iter().map(|p| p.0).collect::<Vec<usize>>(),
        vec![0, 1, 5]
    );

    assert_eq!(
        bfs(graph, 0, 0)
            .iter()
            .map(|p| p.1.clone())
            .collect::<Vec<String>>(),
        vec!["0"]
    );
    assert_eq!(
        bfs(graph, 0, 1)
            .iter()
            .map(|p| p.1.clone())
            .collect::<Vec<String>>(),
        vec!["0", "1"]
    );
    assert_eq!(
        bfs(graph, 0, 2)
            .iter()
            .map(|p| p.1.clone())
            .collect::<Vec<String>>(),
        vec!["0", "1", "2"]
    );
    assert_eq!(
        bfs(graph, 0, 3)
            .iter()
            .map(|p| p.1.clone())
            .collect::<Vec<String>>(),
        Vec::<String>::new()
    );
    assert_eq!(
        bfs(graph, 0, 4)
            .iter()
            .map(|p| p.1.clone())
            .collect::<Vec<String>>(),
        vec!["0", "1", "5", "4"]
    );
    assert_eq!(
        bfs(graph, 0, 5)
            .iter()
            .map(|p| p.1.clone())
            .collect::<Vec<String>>(),
        vec!["0", "1", "5"]
    );
}

#[test]
fn should_find_shortest_path_using_dijkstra() {
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

    assert_eq!(
        dijkstra(graph, 0, 3)
            .iter()
            .map(|i| i.0)
            .collect::<Vec<_>>(),
        vec![0, 5, 4, 3]
    );

    assert_eq!(
        dijkstra(graph, 0, 3)
            .iter()
            .map(|i| i.1.clone())
            .collect::<Vec<_>>(),
        vec!["A", "F", "E", "D"]
    );

    assert_eq!(
        dijkstra(graph, 0, 3)
            .iter()
            .map(|i| i.2)
            .collect::<Vec<_>>(),
        vec![0.0, 9.5, 18.0, 20.5]
    );

    let graph = &Graph::from_adjacency_matrix_with_labels(
        ["A", "B", "C", "D", "E", "F"],
        [
            [None, Some(6.5), None, None, None, Some(9.5)],
            [None, None, Some(-1.5), None, None, None],
            [None, None, None, None, None, Some(2.5)],
            [None, None, Some(4.5), None, None, None],
            [None, None, None, Some(-2.5), None, None],
            [None, None, None, None, Some(0.5), None],
        ],
    );

    assert_eq!(
        dijkstra(graph, 0, 3)
            .iter()
            .map(|i| i.0)
            .collect::<Vec<_>>(),
        vec![0, 1, 2, 5, 4, 3]
    );

    assert_eq!(
        dijkstra(graph, 0, 3)
            .iter()
            .map(|i| i.1.clone())
            .collect::<Vec<_>>(),
        vec!["A", "B", "C", "F", "E", "D"]
    );

    assert_eq!(
        dijkstra(graph, 0, 3)
            .iter()
            .map(|i| i.2)
            .collect::<Vec<_>>(),
        vec![0.0, 6.5, 5.0, 7.5, 8.0, 5.5]
    );

    let graph = &Graph::from_adjacency_matrix_with_labels(
        ["A", "B", "C"],
        [
            [None, Some(6.5), None],
            [Some(6.5), None, None],
            [None, None, None],
        ],
    );

    assert_eq!(dijkstra(graph, 0, 2), vec![]);
}
