use crate::structure::matrix::matrix_trait::MatrixDataTrait;
use crate::structure::matrix::Matrix;
use std::collections::{BTreeMap, HashMap, VecDeque};

/// A collection of popular heuristic functions for a* algorithm
pub mod heuristics;

fn validate<const ROWS: usize, const COLS: usize>(
    source: (usize, usize),
    destination: (usize, usize),
) {
    if source.0 >= ROWS || source.1 >= COLS || destination.0 >= ROWS || destination.1 >= COLS {
        panic!(
            "Invalid start({},{}) or end({},{}) node, available nodes (0,0 - {},{})",
            source.0,
            source.1,
            destination.0,
            destination.1,
            ROWS - 1,
            COLS - 1
        );
    }
}

fn find_neighbours<T: MatrixDataTrait, const ROWS: usize, const COLS: usize>(
    matrix: &Matrix<T, ROWS, COLS>,
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
    .iter()
    .filter(|p| matrix[**p] > T::zero())
    .map(|p| p.clone())
    .collect()
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

    for next in find_neighbours(matrix, node) {
        if visited.get(&next).is_none() || visited.get(&next).unwrap().clone() == false {
            if dfs_visit(matrix, next.clone(), end, path, visited) {
                return true;
            }
        }
    }

    path.pop();
    false
}

fn reconstruct_path<T: MatrixDataTrait, const ROWS: usize, const COLS: usize>(
    parents: HashMap<(usize, usize), (usize, usize)>,
    costs: HashMap<(usize, usize), T>,
    node: (usize, usize),
) -> Vec<((usize, usize), T)> {
    let mut current = node;
    let mut path = vec![(current, *costs.get(&current).unwrap())];
    while let Some(&parent) = parents.get(&current) {
        path.push((parent, *costs.get(&parent).unwrap()));
        current = parent;
    }
    path.reverse();
    path
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
    validate::<ROWS, COLS>(source, destination);

    let mut visited: HashMap<(usize, usize), bool> = HashMap::with_capacity(ROWS * COLS);

    let mut path: Vec<(usize, usize)> = vec![];
    if dfs_visit(matrix, source, destination, &mut path, &mut visited) {
        path
    } else {
        Vec::new()
    }
}

/// Performs a breadth-first search (BFS) on a matrix to find a path from the source node
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
pub fn bfs<T: MatrixDataTrait, const ROWS: usize, const COLS: usize>(
    matrix: &Matrix<T, ROWS, COLS>,
    source: (usize, usize),
    destination: (usize, usize),
) -> Vec<(usize, usize)> {
    validate::<ROWS, COLS>(source, destination);

    let mut visited: HashMap<(usize, usize), bool> = HashMap::with_capacity(ROWS * COLS);
    let mut parent: BTreeMap<(usize, usize), (usize, usize)> = BTreeMap::new();
    let mut queue = VecDeque::new();

    visited.insert(source, true);
    queue.push_front(source);

    while let Some(current) = queue.pop_back() {
        if current == destination {
            let mut path = Vec::<(usize, usize)>::new();
            let mut current = destination;
            while parent.contains_key(&current) {
                path.push(current);
                current = parent[&current];
            }
            path.push(current);
            path.reverse();
            return path;
        }

        for next in find_neighbours(matrix, current) {
            if visited.get(&next).is_none() || visited.get(&next).unwrap().clone() == false {
                visited.insert(next, true);
                parent.insert(next, current);
                queue.push_front(next);
            }
        }
    }

    Vec::new()
}

/// Performs Dijkstra's algorithm on a matrix to find the shortest path from the source node
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
/// A vector of tuples representing the path from the source node to the destination node,
/// along with the cost associated with each node. If no path is found, returns an empty vector.
///
/// # Panics
///
/// Panics if the source or destination nodes are out of the matrix bounds.
pub fn dijkstra<T: MatrixDataTrait, const ROWS: usize, const COLS: usize>(
    matrix: &Matrix<T, ROWS, COLS>,
    source: (usize, usize),
    destination: (usize, usize),
) -> Vec<((usize, usize), T)> {
    validate::<ROWS, COLS>(source, destination);

    let mut parents: HashMap<(usize, usize), (usize, usize)> = HashMap::with_capacity(ROWS * COLS);
    let mut costs: HashMap<(usize, usize), T> = HashMap::with_capacity(ROWS * COLS);
    let mut explored: HashMap<(usize, usize), bool> = HashMap::with_capacity(ROWS * COLS);

    costs.insert(source, T::zero());
    let mut current = source;

    while current.0 < ROWS && current.1 < COLS {
        if current == destination {
            return reconstruct_path::<T, ROWS, COLS>(parents, costs, current);
        }

        explored.insert(current, true);
        let mut next = (usize::MAX, usize::MAX);
        let mut next_weight = None;

        let neighbours = find_neighbours(matrix, current);

        if neighbours.is_empty() {
            return Vec::new();
        }

        for neighbour in find_neighbours(matrix, current) {
            let weight = matrix[neighbour];

            let new_cost = *costs.get(&current).unwrap() + weight;
            match costs.get(&neighbour) {
                None => {
                    costs.insert(neighbour, new_cost);
                    parents.insert(neighbour, current);
                }
                Some(prev_cost) if new_cost < *prev_cost => {
                    costs.insert(neighbour, new_cost);
                    parents.insert(neighbour, current);
                }
                _ => {}
            }
            if (explored.get(&neighbour).is_none()
                || explored.get(&neighbour).unwrap().clone() == false)
                && (next_weight.is_none() || new_cost < next_weight.unwrap())
            {
                next_weight = Some(new_cost);
                next = neighbour;
            }
        }
        current = next;
    }

    Vec::new()
}

/// Performs the A* algorithm on a matrix to find the shortest path from the source node
/// to the destination node using a heuristic function.
///
/// # Arguments
///
/// * `matrix` - A reference to the matrix to be searched.
/// * `source` - A tuple representing the starting node (row, column).
/// * `destination` - A tuple representing the ending node (row, column).
/// * `heu` - A heuristic function that takes two nodes and returns a cost.
///
/// # Returns
///
/// A vector of tuples representing the path from the source node to the destination node,
/// along with the cost associated with each node. If no path is found, returns an empty vector.
///
/// # Panics
///
/// Panics if the source or destination nodes are out of the matrix bounds.
pub fn a_star<T: MatrixDataTrait, const ROWS: usize, const COLS: usize, H>(
    matrix: &Matrix<T, ROWS, COLS>,
    source: (usize, usize),
    destination: (usize, usize),
    heu: H,
) -> Vec<((usize, usize), T)>
where
    T: MatrixDataTrait,
    H: Fn((usize, usize), (usize, usize)) -> T,
{
    validate::<ROWS, COLS>(source, destination);

    let mut parents: HashMap<(usize, usize), (usize, usize)> = HashMap::with_capacity(ROWS * COLS);
    let mut costs: HashMap<(usize, usize), T> = HashMap::with_capacity(ROWS * COLS);
    let mut explored: HashMap<(usize, usize), bool> = HashMap::with_capacity(ROWS * COLS);

    costs.insert(source, T::zero());
    let mut current = source;

    while current.0 < ROWS && current.1 < COLS {
        if current == destination {
            return reconstruct_path::<T, ROWS, COLS>(parents, costs, current);
        }

        explored.insert(current, true);
        let mut next = (usize::MAX, usize::MAX);
        let mut next_heu = None;

        let neighbours = find_neighbours(matrix, current);

        if neighbours.is_empty() {
            return Vec::new();
        }

        for neighbour in find_neighbours(matrix, current) {
            let weight = matrix[neighbour];

            let new_cost = *costs.get(&current).unwrap() + weight;
            let new_heu = new_cost + heu(neighbour, destination);
            match costs.get(&neighbour) {
                None => {
                    costs.insert(neighbour, new_cost);
                    parents.insert(neighbour, current);
                }
                Some(prev_cost) if new_cost < *prev_cost => {
                    costs.insert(neighbour, new_cost);
                    parents.insert(neighbour, current);
                }
                _ => {}
            }
            if (explored.get(&neighbour).is_none()
                || explored.get(&neighbour).unwrap().clone() == false)
                && (next_heu.is_none() || new_heu < next_heu.unwrap())
            {
                next_heu = Some(new_heu);
                next = neighbour;
            }
        }
        current = next;
    }

    Vec::new()
}
