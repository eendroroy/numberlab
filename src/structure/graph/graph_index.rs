use crate::structure::graph::graph_trait::GraphWeightTrait;
use crate::structure::graph::Graph;
use std::ops::{Index, IndexMut};

impl<W: GraphWeightTrait, const NODES: usize> Index<(usize, usize)> for Graph<W, NODES> {
    type Output = Option<W>;

    /// Returns a reference to the edge value at the given (row, column) index.
    ///
    /// # Arguments
    ///
    /// * `index` - A tuple containing the row and column indices.
    ///
    /// # Returns
    ///
    /// * `&Self::Output` - A reference to the edge value, which is an `Option<W>`.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::graph::Graph;
    ///
    /// let graph = Graph::<f32, 6>::new_with_edges([
    ///     [None, Some(1.0), None, None, None, None],
    ///     [None, None, Some(1.0), None, None, Some(1.0)],
    ///     [None, None, None, None, None, Some(1.0)],
    ///     [None, None, Some(1.0), None, None, None],
    ///     [None, None, None, None, None, Some(1.0)],
    ///     [None, None, None, None, Some(1.0), None],
    /// ]);
    /// assert_eq!(graph[(0, 1)], Some(1.0));
    /// assert_eq!(graph[(0, 2)], None);
    /// ```
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.edges[index.0][index.1]
    }
}

impl<W: GraphWeightTrait, const NODES: usize> IndexMut<(usize, usize)> for Graph<W, NODES> {
    /// Returns a mutable reference to the edge value at the given (row, column) index.
    ///
    /// # Arguments
    ///
    /// * `index` - A tuple containing the row and column indices.
    ///
    /// # Returns
    ///
    /// * `&mut Self::Output` - A mutable reference to the edge value, which is an `Option<W>`.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::graph::Graph;
    ///
    /// let mut graph = Graph::<f32, 6>::new_with_edges([
    ///     [None, Some(1.0), None, None, None, None],
    ///     [None, None, Some(1.0), None, None, Some(1.0)],
    ///     [None, None, None, None, None, Some(1.0)],
    ///     [None, None, Some(1.0), None, None, None],
    ///     [None, None, None, None, None, Some(1.0)],
    ///     [None, None, None, None, Some(1.0), None],
    /// ]);
    /// graph[(0, 0)] = Some(1.0);
    /// assert_eq!(graph[(0, 0)], Some(1.0));
    /// ```
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.edges[index.0][index.1]
    }
}
