use crate::structure::matrix::matrix_trait::MatrixDataTrait;
use crate::structure::matrix::Matrix;
use std::collections::HashMap;

fn find_neighbours<const ROWS: usize, const COLS: usize>(
    node: (usize, usize),
) -> Vec<(usize, usize)> {
    match node {
        (r, c) if (r, c) == (0, 0) => vec![(1, 0), (0, 1)], // top-left
        (r, c) if (r, c) == (ROWS - 1, 0) => vec![(r - 1, 0), (r, 1)], // bottom-left
        (r, c) if (r, c) == (ROWS - 1, COLS - 1) => vec![(r - 1, c), (r, c - 1)], // bottom-right
        (r, c) if (r, c) == (0, COLS - 1) => vec![(1, c), (0, c - 1)], // top-right
        (r, c) if r == 0 => vec![(r + 1, c), (r, c + 1), (r, c - 1)], // top
        (r, c) if c == 0 => vec![(r + 1, c), (r, c + 1), (r - 1, c)], // left
        (r, c) if r == ROWS - 1 => vec![(r, c + 1), (r - 1, c), (r, c - 1)], // bottom
        (r, c) if c == COLS - 1 => vec![(r - 1, c), (r + 1, c), (r, c - 1)], // right
        (r, c) => vec![(r + 1, c), (r, c + 1), (r - 1, c), (r, c - 1)], // inner
    }
}

fn dfs_visit<W: MatrixDataTrait, const ROWS: usize, const COLS: usize>(
    matrix: &Matrix<W, ROWS, COLS>,
    node: (usize, usize),
    end: (usize, usize),
    path: &mut Vec<(usize, usize)>,
    visited: &mut HashMap<(usize, usize), bool>,
) -> bool {
    if node == end {
        path.push(node);
        return true;
    }

    visited.insert(node, true);
    path.push(node);

    let neighbours = find_neighbours::<ROWS, COLS>(node);

    for next in neighbours {
        if matrix[next] > W::zero()
            && (visited.get(&next).is_none() || visited.get(&next).unwrap().clone() == false)
        {
            if dfs_visit(matrix, next.clone(), end, path, visited) {
                return true;
            }
        }
    }

    path.pop();
    visited.insert(node, false);
    false
}

/// Performs a depth-first search (DFS) on a matrix to find a path from the source node
/// to the destination node.
///
/// # Arguments
///
/// * `matrix` - A reference to the matrix to be searched.
/// * `source` - A tuple representing the starting node (row, column).
/// * `destination` - A tuple representing the ending node (row, column).
///
/// # Returns
///
/// A vector of tuples representing the path from the source node to the destination node.
/// If no path is found, returns an empty vector.
///
/// # Panics
///
/// Panics if the source or destination nodes are out of the matrix bounds.
pub fn dfs<T: MatrixDataTrait, const ROWS: usize, const COLS: usize>(
    matrix: &Matrix<T, ROWS, COLS>,
    source: (usize, usize),
    destination: (usize, usize),
) -> Vec<(usize, usize)> {
    if source.0 >= ROWS || source.1 >= COLS || destination.0 >= ROWS || destination.1 >= COLS {
        panic!(
            "Invalid start({},{}) or end({},{}) node, available nodes (0,0 - {},{})",
            source.0, source.1, destination.0, destination.1, ROWS, COLS
        );
    }
    let mut visited: HashMap<(usize, usize), bool> = HashMap::with_capacity(ROWS * COLS);

    let mut path: Vec<(usize, usize)> = vec![];
    if dfs_visit(matrix, source, destination, &mut path, &mut visited) {
        path
    } else {
        Vec::new()
    }
}
