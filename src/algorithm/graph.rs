use crate::structure::graph::graph_trait::GraphWeightTrait;
use crate::structure::graph::Graph;
use std::collections::{BTreeMap, HashMap, VecDeque};

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
            "Invalid start ({}) or end node ({}), available nodes (0 - {})",
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
/// let graph = &Graph::from(
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
pub fn dijkstra<W: GraphWeightTrait, const NODES: usize>(
    graph: &Graph<W, NODES>,
    source: usize,
    destination: usize,
) -> Vec<(usize, String, W)> {
    let mut parents: HashMap<usize, usize> = HashMap::with_capacity(NODES);

    let mut costs: [Option<W>; NODES] = [None; NODES];
    let mut explored: [bool; NODES] = [false; NODES];

    costs[source] = Some(W::zero());

    let mut current = source;

    while current < NODES {
        if destination == current {
            let mut path = vec![(current, graph[current].clone(), costs[current].unwrap())];
            while parents.contains_key(&current) {
                let parent = parents[&current];
                path.push((parent, graph[parent].clone(), costs[parent].unwrap()));

                current = parent;
            }

            path.reverse();

            return path;
        }
        explored[current] = true;
        let mut to_visit = usize::MAX;
        let mut to_visit_cost: Option<W> = None;
        for adj in 0..NODES {
            match graph[(current, adj)] {
                Some(weight) => {
                    match costs[adj] {
                        None => {
                            costs[adj] = Some(costs[current].unwrap() + weight);
                            parents.insert(adj, current);
                        }
                        Some(prev_cost) => {
                            let new_cost = costs[current].unwrap() + weight;
                            if new_cost <= prev_cost {
                                costs[adj] = Some(new_cost);
                                parents.insert(adj, current);
                            }
                        }
                    }
                    if costs[adj].is_some()
                        && !explored[adj]
                        && (to_visit_cost.is_none() || to_visit_cost.unwrap() > costs[adj].unwrap())
                    {
                        to_visit_cost = costs[adj];
                        to_visit = adj;
                    }
                }
                None => {}
            }
        }
        current = to_visit;
    }

    Vec::<(usize, String, W)>::new()
}
