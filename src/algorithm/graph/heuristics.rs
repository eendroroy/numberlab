use crate::structure::graph::graph_trait::GraphWeightTrait;

/// Manhattan distance heuristic for A*
pub fn manhattan_heuristic<W: GraphWeightTrait + From<i32>>(
    node: usize,
    goal: usize,
    nodes: usize,
) -> W {
    let ((x1, y1), (x2, y2)) = get_points(node, goal, nodes);
    W::from((x1 - x2).abs() + (y1 - y2).abs())
}

/// Euclidean distance heuristic for A*
pub fn euclidean_heuristic<W: GraphWeightTrait + From<i32>>(
    node: usize,
    goal: usize,
    nodes: usize,
) -> W {
    let ((x1, y1), (x2, y2)) = get_points(node, goal, nodes);
    W::from(((x1 as f64 - x2 as f64).powi(2) + (y1 as f64 - y2 as f64).powi(2)).sqrt() as i32)
}

/// Chebyshev distance heuristic for A*
pub fn chebyshev_heuristic<W: GraphWeightTrait + From<i32>>(
    node: usize,
    goal: usize,
    nodes: usize,
) -> W {
    let ((x1, y1), (x2, y2)) = get_points(node, goal, nodes);
    W::from((x1 - x2).abs().max((y1 - y2).abs()))
}

/// Octile distance heuristic for A*
pub fn octile_heuristic<W: GraphWeightTrait + From<i32>>(
    node: usize,
    goal: usize,
    nodes: usize,
) -> W {
    let ((x1, y1), (x2, y2)) = get_points(node, goal, nodes);
    let (dx, dy) = ((x1 - x2).abs(), (y1 - y2).abs());
    W::from((dx + dy) + ((2 - 2_i32.isqrt()) * dx.min(dy)))
}

/// Dijkstra's Heuristic
pub fn dijkstra_heuristic<W: GraphWeightTrait>(_: usize, _: usize, _: usize) -> W {
    W::zero()
}

fn get_points(node: usize, goal: usize, nodes: usize) -> ((i32, i32), (i32, i32)) {
    (
        ((node % nodes) as i32, (node / nodes) as i32),
        ((goal % nodes) as i32, (goal / nodes) as i32),
    )
}
