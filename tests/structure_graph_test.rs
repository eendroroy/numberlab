use numberlab::structure::graph::{bfs, dfs, Graph};

#[test]
#[should_panic(expected = "Invalid start (6) or end node (0), available nodes (0 - 5)")]
fn should_panic_if_source_invalid_for_dfs() {
    let graph = &Graph::<f32, 6>::new();
    dfs(graph, 6, 0);
}

#[test]
#[should_panic(expected = "Invalid start (0) or end node (6), available nodes (0 - 5)")]
fn should_panic_if_destination_invalid_for_dfs() {
    let graph = &Graph::<f32, 6>::new();
    dfs(graph, 0, 6);
}

#[test]
#[should_panic(expected = "Invalid start (6) or end node (0), available nodes (0 - 5)")]
fn should_panic_if_source_invalid_for_bfs() {
    let graph = &Graph::<f32, 6>::new();
    bfs(graph, 6, 0);
}

#[test]
#[should_panic(expected = "Invalid start (0) or end node (6), available nodes (0 - 5)")]
fn should_panic_if_destination_invalid_for_bfs() {
    let graph = &Graph::<f32, 6>::new();
    bfs(graph, 0, 6);
}

#[test]
fn should_find_path_from_source_to_destination_using_dfs() {
    let graph = &Graph::from_edges([
        [None, Some(1.0), None, None, None, None],
        [None, None, Some(1.0), None, None, Some(1.0)],
        [None, None, None, None, None, Some(1.0)],
        [None, None, Some(1.0), None, None, None],
        [None, None, None, None, None, Some(1.0)],
        [None, None, None, None, Some(1.0), None],
    ]);

    assert_eq!(dfs(graph, 0, 0), vec![0]);
    assert_eq!(dfs(graph, 0, 1), vec![0, 1]);
    assert_eq!(dfs(graph, 0, 2), vec![0, 1, 2]);
    assert_eq!(dfs(graph, 0, 3), vec![]);
    assert_eq!(dfs(graph, 0, 4), vec![0, 1, 2, 5, 4]);
    assert_eq!(dfs(graph, 0, 5), vec![0, 1, 2, 5]);
}

#[test]
fn should_find_path_from_source_to_destination_using_bfs() {
    let graph = &Graph::from_edges([
        [None, Some(1.0), None, None, None, None],
        [None, None, Some(1.0), None, None, Some(1.0)],
        [None, None, None, None, None, Some(1.0)],
        [None, None, Some(1.0), None, None, None],
        [None, None, None, None, None, Some(1.0)],
        [None, None, None, None, Some(1.0), None],
    ]);

    assert_eq!(bfs(graph, 0, 0), vec![0]);
    assert_eq!(bfs(graph, 0, 1), vec![0, 1]);
    assert_eq!(bfs(graph, 0, 2), vec![0, 1, 2]);
    assert_eq!(bfs(graph, 0, 3), vec![]);
    assert_eq!(bfs(graph, 0, 4), vec![0, 1, 5, 4]);
    assert_eq!(bfs(graph, 0, 5), vec![0, 1, 5]);
}

#[test]
fn should_return_distance_from_node_to_node() {
    let graph = &mut Graph::from_edges([
        [None, Some(1.0), None, None, None, None],
        [None, None, Some(1.0), None, None, Some(1.0)],
        [None, None, None, None, None, Some(1.0)],
        [None, None, Some(1.0), None, None, None],
        [None, None, None, None, None, Some(1.0)],
        [None, None, None, None, Some(1.0), None],
    ]);

    graph[(0, 0)] = Some(1.0);
    graph[(0, 1)] = Some(1.0);
    graph[(4, 1)] = Some(1.0);
    graph[(4, 4)] = Some(1.0);
    graph[(1, 2)] = None;

    assert_eq!(graph[(0, 0)], Some(1.0));
    assert_eq!(graph[(0, 1)], Some(1.0));
    assert_eq!(graph[(4, 1)], Some(1.0));
    assert_eq!(graph[(4, 4)], Some(1.0));
    assert_eq!(graph[(1, 2)], None);
}
