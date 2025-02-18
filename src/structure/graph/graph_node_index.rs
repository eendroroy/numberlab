use crate::structure::graph::graph_trait::GraphWeightTrait;
use crate::structure::graph::Graph;
use std::ops::{Index, IndexMut};

impl<W: GraphWeightTrait, const NODES: usize> Index<usize> for Graph<W, NODES> {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<W: GraphWeightTrait, const NODES: usize> IndexMut<usize> for Graph<W, NODES> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}
