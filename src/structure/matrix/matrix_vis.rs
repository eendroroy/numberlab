use crate::structure::matrix::matrix_trait::MatrixDataTrait;
use crate::structure::matrix::Matrix;
use core::array::from_fn as array_fn;

/// A structure representing a matrix visualization with a fixed number of rows and columns.
///
/// # Type Parameters
///
/// * `ROWS` - The number of rows in the matrix.
/// * `COLS` - The number of columns in the matrix. Defaults to the number of rows if not specified.
#[derive(Debug, Clone)]
pub struct MatrixVis<const ROWS: usize, const COLS: usize = ROWS> {
    pub(crate) data: [[String; COLS]; ROWS],
}

impl<const ROWS: usize, const COLS: usize> MatrixVis<ROWS, COLS> {
    /// Creates a new `MatrixVis` instance from a given path, with specified characters for the path and background.
    ///
    /// # Arguments
    ///
    /// * `path` - A vector of tuples representing the coordinates of the path in the matrix.
    /// * `w` - A character to represent the path in the matrix.
    /// * `b` - A character to represent the background in the matrix.
    ///
    /// # Returns
    ///
    /// A new `MatrixVis` instance with the path and background characters set accordingly.
    pub fn from_path(path: Vec<(usize, usize)>, w: char, b: char) -> Self {
        let mut data: [[String; COLS]; ROWS] = array_fn(|_| array_fn(|_| String::from(b)));
        path.iter().for_each(|n| data[n.0][n.1] = String::from(w));
        Self { data }
    }

    /// Creates a new `MatrixVis` instance from a given matrix, with specified characters for the path and background.
    ///
    /// # Arguments
    ///
    /// * `matrix` - A reference to a `Matrix` instance implementing the `MatrixDataTrait`.
    /// * `w` - A character to represent the path in the matrix.
    /// * `b` - A character to represent the background in the matrix.
    ///
    /// # Returns
    ///
    /// A new `MatrixVis` instance with the path and background characters set accordingly.
    pub fn from_mat<T: MatrixDataTrait>(matrix: &Matrix<T, ROWS, COLS>, w: char, b: char) -> Self {
        let mut data: [[String; COLS]; ROWS] = array_fn(|_| array_fn(|_| String::from(b)));
        for i in 0..ROWS {
            for j in 0..COLS {
                data[i][j] = if matrix[(i, j)] > T::zero() {
                    String::from(w)
                } else {
                    String::from(b)
                };
            }
        }
        Self { data }
    }

    /// Highlights a given path in the matrix visualization using a specified character and formatting function.
    ///
    /// # Arguments
    ///
    /// * `path` - A vector of tuples representing the coordinates of the path in the matrix.
    /// * `w` - A character to represent the path in the matrix.
    /// * `fmt` - A formatting function to apply to the path character.
    ///
    /// # Returns
    ///
    /// A new `MatrixVis` instance with the highlighted path.
    pub fn highlight_path<F>(&self, path: Vec<(usize, usize)>, w: char, fmt: F) -> Self
    where
        F: Fn(String) -> String,
    {
        let mut cloned = self.clone();
        path.iter()
            .for_each(|n| cloned.data[n.0][n.1] = fmt(String::from(w)));
        cloned
    }
}

impl<const ROWS: usize, const COLS: usize> std::fmt::Display for MatrixVis<ROWS, COLS> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        writeln!(f).expect("Failed to write to formatter");
        for i in 0..ROWS {
            for j in 0..COLS {
                write!(f, "{}", self.data[i][j]).expect("Failed to write to formatter");
            }
            writeln!(f).expect("Failed to write to formatter");
        }
        Ok(())
    }
}
