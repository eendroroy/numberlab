use numberlab::algorithm::graph::{bfs, dfs};
use numberlab::structure::graph::Graph;

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
    let graph = &Graph::from_edges([
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
