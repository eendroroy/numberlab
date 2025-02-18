use numberlab::structure::graph::Graph;

fn main() {
    let graph = Graph::from(
        ["AA", "BB", "CC", "DD", "EE", "FF"],
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
}
