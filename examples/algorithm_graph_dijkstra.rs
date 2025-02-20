use numberlab::algorithm::graph::{a_star, dijkstra};
use numberlab::algorithm::graph::heuristics::{
    chebyshev_heuristic, dijkstra_heuristic, euclidean_heuristic, manhattan_heuristic,
    octile_heuristic,
};
use numberlab::structure::graph::Graph;

fn main() {
    let graph = &Graph::from_adjacency_matrix([
        [None, Some(4), Some(1), None, None, None, None, None, None, None],
        [Some(4), None, Some(2), Some(1), None, None, None, None, None, None],
        [Some(1), Some(2), None, Some(5), Some(3), None, None, None, None, None],
        [None, Some(1), Some(5), None, Some(2), Some(8), None, None, None, None],
        [None, None, Some(3), Some(2), None, Some(6), Some(7), None, None, None],
        [None, None, None, Some(8), Some(6), None, Some(2), Some(3), None, None],
        [None, None, None, None, Some(7), Some(2), None, Some(5), Some(1), None],
        [None, None, None, None, None, Some(3), Some(5), None, Some(4), Some(2)],
        [None, None, None, None, None, None, Some(1), Some(4), None, Some(6)],
        [None, None, None, None, None, None, None, Some(2), Some(6), None],
    ]);

    println!("{}", graph);
    println!();
    println!("{:?}", dijkstra(graph, 0, 9));
    println!("{:?}", a_star(graph, 0, 9, dijkstra_heuristic));
    println!("{:?}", a_star(graph, 0, 9, manhattan_heuristic));
    println!("{:?}", a_star(graph, 0, 9, octile_heuristic));
    println!("{:?}", a_star(graph, 0, 9, chebyshev_heuristic));
    println!("{:?}", a_star(graph, 0, 9, euclidean_heuristic));
}

