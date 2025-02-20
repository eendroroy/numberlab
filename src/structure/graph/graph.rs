use crate::structure::graph::graph_trait::GraphWeightTrait;
use std::collections::HashMap;

/// A struct representing a graph with a fixed number of nodes and edges.
///
/// # Type Parameters
///
/// * `W` - The type of the weight of the edges.
/// * `NODES` - The number of nodes in the graph.
#[derive(Debug, Clone, PartialEq)]
pub struct Graph<W, const NODES: usize> {
    pub(crate) labels: [String; NODES],
    pub(crate) adjacency: [[Option<W>; NODES]; NODES],
}

impl<W: GraphWeightTrait, const NODES: usize> Graph<W, NODES> {
    /// Creates a new graph with no edges.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::graph::Graph;
    ///
    /// let graph = Graph::<f32, 3>::new();
    /// assert_eq!(graph, Graph::from_adjacency_matrix([[None; 3]; 3]));
    /// ```
    pub fn new() -> Self {
        Self {
            labels: std::array::from_fn(|n| n.to_string()),
            adjacency: [[None; NODES]; NODES],
        }
    }

    /// Creates a new graph with the given nodes and edges.
    ///
    /// # Parameters
    ///
    /// * `labels` - An array of node names.
    /// * `adjacency` - A 2D array representing the edges of the graph.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::graph::Graph;
    ///
    /// let labels = ["A", "B", "C"];
    /// let adjacency = [
    ///     [None, Some(1.0), None],
    ///     [None, None, Some(1.0)],
    ///     [None, None, None],
    /// ];
    /// let graph = Graph::from_adjacency_matrix_with_labels(labels, adjacency);
    ///
    /// assert_eq!(graph[0], "A");
    /// assert_eq!(graph[1], "B");
    /// assert_eq!(graph[2], "C");
    /// assert_eq!(graph[(0, 1)], Some(1.0));
    /// assert_eq!(graph[(1, 2)], Some(1.0));
    /// assert_eq!(graph[(2, 0)], None);
    /// ```
    pub fn from_adjacency_matrix_with_labels(
        labels: [&str; NODES],
        adjacency: [[Option<W>; NODES]; NODES],
    ) -> Self {
        Self {
            labels: std::array::from_fn(|i| labels[i].to_string()),
            adjacency,
        }
    }

    /// Creates a new graph with the given edges.
    ///
    /// # Parameters
    ///
    /// * `adjacency` - A 2D array representing the edges of the graph.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::graph::Graph;
    ///
    /// let adjacency = [
    ///     [None, Some(1.0), None],
    ///     [None, None, Some(1.0)],
    ///     [None, None, None],
    /// ];
    /// let graph = Graph::from_adjacency_matrix(adjacency);
    /// assert_eq!(graph[(0, 1)], Some(1.0));
    /// assert_eq!(graph[(1, 2)], Some(1.0));
    /// assert_eq!(graph[(2, 0)], None);
    /// ```
    pub fn from_adjacency_matrix(adjacency: [[Option<W>; NODES]; NODES]) -> Self {
        Self {
            labels: std::array::from_fn(|n| n.to_string()),
            adjacency,
        }
    }

    /// Creates a new graph from a list of edges.
    ///
    /// # Parameters
    ///
    /// * `edges` - A vector of tuples representing the edges of the graph, where each tuple contains
    ///   the indices of the start and end nodes and the weight of the edge.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::graph::Graph;
    ///
    /// let edges = vec![(0, 1, 1.0), (1, 2, 1.0)];
    /// let graph = Graph::<f64, 3>::from_edges(edges);
    ///
    /// assert_eq!(graph[(0, 1)], Some(1.0));
    /// assert_eq!(graph[(1, 2)], Some(1.0));
    /// assert_eq!(graph[(2, 0)], None);
    /// ```
    pub fn from_edges(edges: Vec<(usize, usize, W)>) -> Self {
        let mut adjacency: [[Option<W>; NODES]; NODES] = [[None; NODES]; NODES];
        edges
            .iter()
            .for_each(|edge| adjacency[edge.0][edge.1] = Some(edge.2));
        Self {
            labels: std::array::from_fn(|n| n.to_string()),
            adjacency,
        }
    }

    /// Creates a new graph with the given edges and labels.
    ///
    /// # Parameters
    ///
    /// * `labels` - An array of node names.
    /// * `edges` - A vector of tuples representing the edges of the graph, where each tuple contains
    ///   the indices of the start and end nodes and the weight of the edge.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::graph::Graph;
    ///
    /// let labels = ["A", "B", "C"];
    /// let edges = vec![(0, 1, 1.0), (1, 2, 1.0)];
    /// let graph = Graph::from_edges_with_labels(labels, edges);
    ///
    /// assert_eq!(graph[0], "A");
    /// assert_eq!(graph[1], "B");
    /// assert_eq!(graph[2], "C");
    /// assert_eq!(graph[(0, 1)], Some(1.0));
    /// assert_eq!(graph[(1, 2)], Some(1.0));
    /// assert_eq!(graph[(2, 0)], None);
    /// ```
    pub fn from_edges_with_labels(labels: [&str; NODES], edges: Vec<(usize, usize, W)>) -> Self {
        let mut adjacency: [[Option<W>; NODES]; NODES] = [[None; NODES]; NODES];
        edges
            .iter()
            .for_each(|edge| adjacency[edge.0][edge.1] = Some(edge.2));
        Self {
            labels: std::array::from_fn(|i| labels[i].to_string()),
            adjacency,
        }
    }
}

impl<W: GraphWeightTrait, const NODES: usize> std::fmt::Display for Graph<W, NODES> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        let mut lengths = HashMap::<usize, usize>::new();

        for j in 0..NODES {
            let header_length = format!("N{}", j).len();
            let max_entry_length = (0..NODES)
                .map(|i| match self[(i, j)] {
                    None => 1,
                    Some(w) => w.to_string().len(),
                })
                .max()
                .unwrap_or(1);
            lengths.insert(j, header_length.max(max_entry_length));
        }

        write!(f, "{:>width$} ", " ", width = lengths[&0] + 1)
            .expect("Failed to write to formatter");
        for j in 0..NODES {
            write!(f, " {:>width$}", self[j], width = lengths[&j] + 1)
                .expect("Failed to write to formatter");
        }
        writeln!(f).expect("Failed to write to formatter");

        write!(f, "{:->width$}-", "-", width = lengths[&0] + 1)
            .expect("Failed to write to formatter");
        for j in 0..NODES {
            write!(f, "{:->width$}-", "-", width = lengths[&j] + 1)
                .expect("Failed to write to formatter");
        }
        writeln!(f)?;

        for i in 0..NODES {
            write!(f, "{:>width$} |", self[i], width = lengths[&0] + 1)
                .expect("Failed to write to formatter");
            for j in 0..NODES {
                let w = match self.adjacency[i][j] {
                    None => ".".to_string(),
                    Some(w) => w.to_string(),
                };
                write!(f, "{:>width$} ", w, width = lengths[&j] + 1)
                    .expect("Failed to write to formatter");
            }
            writeln!(f).expect("Failed to write to formatter");
        }
        Ok(())
    }
}
