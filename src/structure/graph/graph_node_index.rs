use crate::structure::graph::graph_trait::GraphWeightTrait;
use crate::structure::graph::Graph;
use std::ops::{Index, IndexMut};

impl<W: GraphWeightTrait, const NODES: usize> Index<usize> for Graph<W, NODES> {
    type Output = String;

    /// Returns a reference to the node value at the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the node.
    ///
    /// # Returns
    ///
    /// * `&Self::Output` - A reference to the node value, which is a `String`.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::graph::Graph;
    ///
    /// let graph = Graph::<f32, 6>::new();
    /// assert_eq!(graph[0], "0");
    /// assert_eq!(graph[1], "1");
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<W: GraphWeightTrait, const NODES: usize> IndexMut<usize> for Graph<W, NODES> {
    /// Returns a mutable reference to the node value at the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the node.
    ///
    /// # Returns
    ///
    /// * `&mut Self::Output` - A mutable reference to the node value, which is a `String`.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::graph::Graph;
    ///
    /// let mut graph = Graph::<f32, 6>::new();
    /// graph[0] = "A".to_string();
    /// assert_eq!(graph[0], "A");
    /// ```
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}
