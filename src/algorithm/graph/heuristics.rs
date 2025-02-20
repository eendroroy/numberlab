use crate::structure::graph::graph_trait::GraphWeightTrait;

/// Manhattan distance heuristic for A*
pub fn manhattan_heuristic(node: usize, goal: usize) -> i32 {
    let (x1, y1) = (node % 3, node / 3);
    let (x2, y2) = (goal % 3, goal / 3);
    (x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()
}

/// Euclidean distance heuristic for A*
pub fn euclidean_heuristic(node: usize, goal: usize) -> i32 {
    let (x1, y1) = (node % 3, node / 3);
    let (x2, y2) = (goal % 3, goal / 3);
    ((x1 as f64 - x2 as f64).powi(2) + (y1 as f64 - y2 as f64).powi(2)).sqrt() as i32
}

/// Chebyshev distance heuristic for A*
pub fn chebyshev_heuristic(node: usize, goal: usize) -> i32 {
    let (x1, y1) = (node % 3, node / 3);
    let (x2, y2) = (goal % 3, goal / 3);
    (x1 as i32 - x2 as i32)
        .abs()
        .max((y1 as i32 - y2 as i32).abs())
}

/// Octile distance heuristic for A*
pub fn octile_heuristic(node: usize, goal: usize) -> i32 {
    let (x1, y1) = (node % 3, node / 3);
    let (x2, y2) = (goal % 3, goal / 3);
    let dx = (x1 as i32 - x2 as i32).abs();
    let dy = (y1 as i32 - y2 as i32).abs();
    (dx + dy) + ((2 - 2_i32.isqrt()) * dx.min(dy))
}

/// Dijkstra's Heuristic
pub fn dijkstra_heuristic<W: GraphWeightTrait>(_: usize, _: usize) -> W {
    W::zero()
}

