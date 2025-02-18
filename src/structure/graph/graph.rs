use crate::structure::graph::graph_trait::GraphWeightTrait;
use std::collections::HashMap;

/// A struct representing a graph with a fixed number of nodes and edges.
///
/// # Type Parameters
///
/// * `W` - The type of the weight of the edges.
/// * `NODES` - The number of nodes in the graph.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Graph<W, const NODES: usize> {
    pub(crate) edges: [[Option<W>; NODES]; NODES],
}

impl<W: GraphWeightTrait, const NODES: usize> Graph<W, NODES> {
    /// Creates a new graph with no edges.
    ///
    /// # Examples
    ///
    /// ```
    /// use numberlab::structure::graph::Graph;
    ///
    /// let graph = Graph::<i32, 3>::new();
    /// assert_eq!(graph, Graph::new_with_edges([[None; 3]; 3]));
    /// ```
    pub fn new() -> Self {
        Self {
            edges: [[None; NODES]; NODES],
        }
    }

    /// Creates a new graph with the given edges.
    ///
    /// # Parameters
    ///
    /// * `edges` - A 2D array representing the edges of the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use numberlab::structure::graph::Graph;
    ///
    /// let edges = [
    ///     [None, Some(1), None],
    ///     [None, None, Some(1)],
    ///     [None, None, None],
    /// ];
    /// let graph = Graph::new_with_edges(edges);
    /// assert_eq!(graph[(0, 1)], Some(1));
    /// assert_eq!(graph[(1, 2)], Some(1));
    /// assert_eq!(graph[(2, 0)], None);
    /// ```
    pub fn new_with_edges(edges: [[Option<W>; NODES]; NODES]) -> Self {
        Self { edges }
    }
}

impl<W: GraphWeightTrait, const NODES: usize> std::fmt::Display for Graph<W, NODES> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        let mut lengths = HashMap::<usize, usize>::new();

        for j in 0..NODES {
            let header_length = format!("N{}", j).len();
            let max_entry_length = (0..NODES)
                .map(|i| match self.edges[i][j] {
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
            write!(f, " {:>width$}", format!("N{}", j), width = lengths[&j] + 1)
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
            write!(
                f,
                "{:>width$} |",
                format!("N{}", i),
                width = lengths[&0] + 1
            )
            .expect("Failed to write to formatter");
            for j in 0..NODES {
                let w = match self.edges[i][j] {
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
