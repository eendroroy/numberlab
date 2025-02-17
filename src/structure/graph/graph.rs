use crate::structure::graph::graph_trait::GraphDistanceTrait;

/// A struct representing a graph with a fixed number of nodes and edges.
///
/// # Type Parameters
///
/// * `D` - The type of the distance or weight of the edges.
/// * `NODES` - The number of nodes in the graph.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Graph<D, const NODES: usize> {
    pub(crate) edges: [[Option<D>; NODES]; NODES],
}

impl<D: GraphDistanceTrait, const NODES: usize> Graph<D, NODES> {
    /// Creates a new graph with no edges.
    ///
    /// # Returns
    ///
    /// A new `Graph` instance with all edges set to `None`.
    pub fn new() -> Self {
        Self {
            edges: [[None; NODES]; NODES],
        }
    }

    /// Creates a new graph with the specified edges.
    ///
    /// # Parameters
    ///
    /// * `edges` - A 2D array representing the edges of the graph. Each element is an `Option`
    ///   containing the distance or weight of the edge.
    ///
    /// # Returns
    ///
    /// A new `Graph` instance with the specified edges.
    pub fn new_with_edges(edges: [[Option<D>; NODES]; NODES]) -> Self {
        Self { edges }
    }
}
