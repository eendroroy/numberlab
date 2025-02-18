use numberlab::structure::graph::Graph;

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
