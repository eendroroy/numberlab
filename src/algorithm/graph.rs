use crate::structure::graph::graph_trait::GraphWeightTrait;
use crate::structure::graph::Graph;
use std::collections::{BTreeMap, VecDeque};

fn dfs_visit<W: GraphWeightTrait, const NODES: usize>(
    graph: &Graph<W, NODES>,
    node: usize,
    end: usize,
    path: &mut Vec<usize>,
    visited: &mut Vec<bool>,
) -> bool {
    if node == end {
        path.push(node);
        return true;
    }

    visited[node] = true;
    path.push(node);

    for next in 0..NODES {
        if graph.edges[node][next].is_some() && !visited[next] {
            if dfs_visit(graph, next, end, path, visited) {
                return true;
            }
        }
    }

    path.pop();
    visited[node] = false;
    false
}

/// Performs a Depth-First Search (DFS) on the graph to find a path from the source node to the destination node.
///
/// # Arguments
///
/// * `graph` - The graph to search.
/// * `source` - The starting node for the search.
/// * `destination` - The target node to reach.
///
/// # Returns
///
/// A vector of node indices representing the path from the source to the destination.
/// If no path is found, returns an empty vector.
///
/// # Panics
///
/// Panics if the source or destination node indices are out of bounds.
pub fn dfs<W: GraphWeightTrait, const NODES: usize>(
    graph: &Graph<W, NODES>,
    source: usize,
    destination: usize,
) -> Vec<usize> {
    if source >= NODES || destination >= NODES {
        panic!(
            "Invalid start ({}) or end node ({}), available nodes (0 - {})",
            source,
            destination,
            NODES - 1
        );
    }

    let mut visited = vec![false; NODES];
    let mut path: Vec<usize> = vec![];
    if dfs_visit(graph, source, destination, &mut path, &mut visited) {
        path
    } else {
        Vec::new()
    }
}

/// Performs a Breadth-First Search (BFS) on the graph to find a path from the source node to the destination node.
///
/// # Arguments
///
/// * `graph` - The graph to search.
/// * `source` - The starting node for the search.
/// * `destination` - The target node to reach.
///
/// # Returns
///
/// A vector of node indices representing the path from the source to the destination.
/// If no path is found, returns an empty vector.
///
/// # Panics
///
/// Panics if the source or destination node indices are out of bounds.
pub fn bfs<W: GraphWeightTrait, const NODES: usize>(
    graph: &Graph<W, NODES>,
    source: usize,
    destination: usize,
) -> Vec<usize> {
    if source >= NODES || destination >= NODES {
        panic!(
            "Invalid start ({}) or end node ({}), available nodes (0 - {})",
            source,
            destination,
            NODES - 1
        );
    }

    let mut visited = vec![false; NODES];
    let mut parent: BTreeMap<usize, usize> = BTreeMap::new();
    let mut queue = VecDeque::new();

    visited[source] = true;
    queue.push_front(source);

    while let Some(current) = queue.pop_back() {
        if current == destination {
            let mut path = Vec::new();
            let mut current = destination;
            while parent.contains_key(&current) {
                path.push(current);
                current = parent[&current];
            }
            path.push(current);
            path.reverse();
            return path;
        }

        for next in 0..NODES {
            if !visited[next] && graph[(current, next)].is_some() {
                visited[next] = true;
                parent.insert(next, current);
                queue.push_front(next);
            }
        }
    }

    vec![]
}
