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
    pub fn new_with_edges(edges: [[Option<D>; NODES]; NODES]) -> Self {
        Self { edges }
    }
}
