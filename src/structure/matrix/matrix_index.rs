use crate::structure::matrix::Matrix;
use std::ops::{Index, IndexMut};

impl<T, const ROWS: usize, const COLS: usize> Index<(usize, usize)> for Matrix<T, ROWS, COLS> {
    type Output = T;

    /// Returns a reference to the element at the specified position.
    ///
    /// # Arguments
    ///
    /// * `index` - A tuple containing the row and column indices.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix = Matrix::from_array([[1, 2], [3, 4]]);
    /// assert_eq!(matrix[(0, 0)], 1);
    /// assert_eq!(matrix[(1, 1)], 4);
    /// ```
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<T, const ROWS: usize, const COLS: usize> IndexMut<(usize, usize)> for Matrix<T, ROWS, COLS> {
    /// Returns a mutable reference to the element at the specified position.
    ///
    /// # Arguments
    ///
    /// * `index` - A tuple containing the row and column indices.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let mut matrix = Matrix::from_array([[1, 2], [3, 4]]);
    /// matrix[(0, 0)] = 5;
    /// assert_eq!(matrix[(0, 0)], 5);
    /// ```
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}
