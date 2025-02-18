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
fn should_return_weight_from_node_to_node() {
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

#[test]
fn should_return_node_label() {
    let graph = &mut Graph::from(
        ["A", "B", "C"],
        [
            [None, Some(1.0), Some(1.0)],
            [Some(1.0), None, Some(1.0)],
            [Some(1.0), Some(1.0), None],
        ],
    );

    graph[0] = "D".to_string();

    assert_eq!(graph[0], "D");
    assert_eq!(graph[1], "B");
    assert_eq!(graph[2], "C");
}

#[test]
fn should_display_graph() {
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
    let dis = "       A   B   C   D   E   F\
    \n----------------------------\
    \n  A |  .   1   .   .   .   2 \
    \n  B |  .   .   3   .   .   . \
    \n  C |  .   .   .   .   .   4 \
    \n  D |  .   .   5   .   .   . \
    \n  E |  .   .   .   6   .   . \
    \n  F |  .   .   .   .   7   . \
    \n";

    assert_eq!(format!("{}", graph), dis);
}
