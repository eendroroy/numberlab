use crate::structure::graph::graph_trait::GraphWeightTrait;
use crate::structure::graph::Graph;
use std::collections::{BTreeMap, HashMap, VecDeque};

/// A collection of popular heuristic functions for a* algorithm
pub mod heuristics;

fn dfs_visit<W: GraphWeightTrait, const NODES: usize>(
    graph: &Graph<W, NODES>,
    node: usize,
    end: usize,
    path: &mut Vec<(usize, String)>,
    visited: &mut Vec<bool>,
) -> bool {
    if node == end {
        path.push((node, graph[node].clone()));
        return true;
    }

    visited[node] = true;
    path.push((node, graph[node].clone()));

    for next in 0..NODES {
        if graph.adjacency[node][next].is_some() && !visited[next] {
            if dfs_visit(graph, next, end, path, visited) {
                return true;
            }
        }
    }

    path.pop();
    false
}

fn reconstruct_path<W: GraphWeightTrait, const NODES: usize>(
    graph: &Graph<W, NODES>,
    parents: HashMap<usize, usize>,
    costs: [Option<W>; NODES],
    node: usize,
) -> Vec<(usize, String, W)> {
    let mut current = node;
    let mut path = vec![(current, graph[current].clone(), costs[current].unwrap())];
    while let Some(&parent) = parents.get(&current) {
        path.push((parent, graph[parent].clone(), costs[parent].unwrap()));
        current = parent;
    }
    path.reverse();
    path
}

/// Performs a Depth-First Search (DFS) on the graph to find a path from the source node to the destination node.
///
/// # Arguments
///
/// * `graph` - The graph to search.
/// * `source` - The starting node index for the search.
/// * `destination` - The target node index to reach.
///
/// # Returns
///
/// A vector of nodes `(index, label)` representing the path from the source to the destination.
/// If no path is found, returns an empty vector.
///
/// # Panics
///
/// Panics if the source or destination node indices are out of bounds.
pub fn dfs<W: GraphWeightTrait, const NODES: usize>(
    graph: &Graph<W, NODES>,
    source: usize,
    destination: usize,
) -> Vec<(usize, String)> {
    if source >= NODES || destination >= NODES {
        panic!(
            "Invalid start({}) or end({}) node, available nodes (0 - {})",
            source,
            destination,
            NODES - 1
        );
    }

    let mut visited = vec![false; NODES];
    let mut path: Vec<(usize, String)> = vec![];
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
/// * `source` - The starting node index for the search.
/// * `destination` - The target node index to reach.
///
/// # Returns
///
/// A vector of nodes `(index, label)` representing the path from the source to the destination.
/// If no path is found, returns an empty vector.
///
/// # Panics
///
/// Panics if the source or destination node indices are out of bounds.
pub fn bfs<W: GraphWeightTrait, const NODES: usize>(
    graph: &Graph<W, NODES>,
    source: usize,
    destination: usize,
) -> Vec<(usize, String)> {
    if source >= NODES || destination >= NODES {
        panic!(
            "Invalid start({}) or end({}) node, available nodes (0 - {})",
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
            let mut path = Vec::<(usize, String)>::new();
            let mut current = destination;
            while parent.contains_key(&current) {
                path.push((current, graph[current].clone()));
                current = parent[&current];
            }
            path.push((current, graph[current].clone()));
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

/// Performs Dijkstra's algorithm on the graph to find the shortest path from the source node to the destination node.
///
/// # Arguments
///
/// * `graph` - The graph to search.
/// * `source` - The starting node index for the search.
/// * `destination` - The target node index to reach.
///
/// # Returns
///
/// A vector of nodes `(index, label, cost)` representing the path from the source to the destination.
/// If no path is found, returns an empty vector.
///
/// # Panics
///
/// Panics if the source or destination node indices are out of bounds.
///
/// # Example
///
/// ```
/// use numberlab::algorithm::graph::dijkstra;
/// use numberlab::structure::graph::Graph;
///
/// let graph = &Graph::from_adjacency_matrix_with_labels(
///     ["A", "B", "C", "D", "E", "F"],
///     [
///         [None, Some(6.5), None, None, None, Some(9.5)],
///         [None, None, Some(1.5), None, None, None],
///         [None, None, None, None, None, Some(2.5)],
///         [None, None, Some(4.5), None, None, None],
///         [None, None, None, Some(2.5), None, None],
///         [None, None, None, None, Some(8.5), None],
///     ],
/// );
/// let path = dijkstra(graph, 0, 3);
/// assert_eq!(path, vec![
///     (0, "A".to_string(), 0.0),
///     (5, "F".to_string(), 9.5),
///     (4, "E".to_string(), 18.0),
///     (3, "D".to_string(), 20.5),
/// ]);
/// ```
//noinspection DuplicatedCode
pub fn dijkstra<W: GraphWeightTrait, const NODES: usize>(
    graph: &Graph<W, NODES>,
    source: usize,
    destination: usize,
) -> Vec<(usize, String, W)> {
    if source >= NODES || destination >= NODES {
        panic!(
            "Invalid start({}) or end({}) node, available nodes (0 - {})",
            source,
            destination,
            NODES - 1
        );
    }

    let mut parents = HashMap::with_capacity(NODES);
    let mut costs = [None; NODES];
    let mut explored = [false; NODES];

    costs[source] = Some(W::zero());
    let mut current = source;

    while current < NODES {
        if current == destination {
            return reconstruct_path(graph, parents, costs, current);
        }

        explored[current] = true;
        let mut next = usize::MAX;
        let mut next_weight = None;

        for adj in 0..NODES {
            if let Some(weight) = graph[(current, adj)] {
                let new_cost = costs[current].unwrap() + weight;
                match costs[adj] {
                    None => {
                        costs[adj] = Some(new_cost);
                        parents.insert(adj, current);
                    }
                    Some(prev_cost) if new_cost < prev_cost => {
                        costs[adj] = Some(new_cost);
                        parents.insert(adj, current);
                    }
                    _ => {}
                }
                if !explored[adj] && (next_weight.is_none() || new_cost < next_weight.unwrap()) {
                    next_weight = Some(new_cost);
                    next = adj;
                }
            }
        }
        current = next;
    }

    Vec::new()
}

/// Performs the A\* algorithm on the graph to find the shortest path from the source node to the destination node.
///
/// # Arguments
///
/// * `graph` - The graph to search.
/// * `source` - The starting node index for the search.
/// * `destination` - The target node index to reach.
/// * `heu` - A heuristic function that estimates the cost from a node to the destination.
///
/// # Returns
///
/// A vector of nodes `(index, label, cost)` representing the path from the source to the destination.
/// If no path is found, returns an empty vector.
///
/// # Panics
///
/// Panics if the source or destination node indices are out of bounds.
///
/// # Example
///
/// ```
/// use numberlab::algorithm::graph::a_star;
/// use numberlab::structure::graph::Graph;
///
/// let graph = &Graph::from_adjacency_matrix_with_labels(
///     ["A", "B", "C", "D", "E", "F"],
///     [
///         [None, Some(6.5), None, None, None, Some(9.5)],
///         [None, None, Some(1.5), None, None, None],
///         [None, None, None, None, None, Some(2.5)],
///         [None, None, Some(4.5), None, None, None],
///         [None, None, None, Some(2.5), None, None],
///         [None, None, None, None, Some(8.5), None],
///     ],
/// );
/// let path = a_star(graph, 0, 3, |a, b| (a as f64 - b as f64).abs());
/// assert_eq!(path, vec![
///     (0, "A".to_string(), 0.0),
///     (5, "F".to_string(), 9.5),
///     (4, "E".to_string(), 18.0),
///     (3, "D".to_string(), 20.5),
/// ]);
/// ```
//noinspection DuplicatedCode
pub fn a_star<W: GraphWeightTrait, const NODES: usize, H: Fn(usize, usize) -> W>(
    graph: &Graph<W, NODES>,
    source: usize,
    destination: usize,
    heu: H,
) -> Vec<(usize, String, W)> {
    if source >= NODES || destination >= NODES {
        panic!(
            "Invalid start({}) or end({}) node, available nodes (0 - {})",
            source,
            destination,
            NODES - 1
        );
    }

    let mut parents = HashMap::with_capacity(NODES);
    let mut costs = [None; NODES];
    let mut explored = [false; NODES];

    costs[source] = Some(W::zero());
    let mut current = source;

    while current < NODES {
        if current == destination {
            return reconstruct_path(graph, parents, costs, current);
        }

        explored[current] = true;
        let mut next = usize::MAX;
        let mut next_heu = None;

        for adj in 0..NODES {
            if let Some(weight) = graph[(current, adj)] {
                let new_cost = costs[current].unwrap() + weight;
                let new_heu = new_cost + heu(adj, destination);
                match costs[adj] {
                    None => {
                        costs[adj] = Some(new_cost);
                        parents.insert(adj, current);
                    }
                    Some(prev_cost) if new_cost < prev_cost => {
                        costs[adj] = Some(new_cost);
                        parents.insert(adj, current);
                    }
                    _ => {}
                }
                if !explored[adj] && (next_heu.is_none() || new_heu < next_heu.unwrap()) {
                    next_heu = Some(new_heu);
                    next = adj;
                }
            }
        }
        current = next;
    }

    Vec::new()
}
