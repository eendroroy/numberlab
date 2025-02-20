use numberlab::structure::graph::Graph;

fn main() {
    let graph = Graph::from_adjacency_matrix_with_labels(
        ["AA", "BB", "CC", "DD", "EE", "FF"],
        [
            [None, Some(1.0), None, None, None, Some(2.0)],
            [None, None, Some(3.0), None, None, None],
            [None, None, None, None, None, Some(4.0)],
            [None, None, Some(5.0), None, None, None],
            [None, None, None, Some(6.0), None, None],
            [None, None, None, None, Some(7.0), None],
        ],
    );

    println!("{}", graph);
}
