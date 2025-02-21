use crate::structure::matrix::matrix_trait::MatrixDataTrait;
use crate::structure::matrix::MatrixVis;
use std::collections::HashMap;

/// A generic Matrix struct that holds a 2D array of elements.
///
/// # Type Parameters
///
/// * `T` - The type of the elements in the matrix.
/// * `ROWS` - The number of rows in the matrix.
/// * `COLS` - The number of columns in the matrix.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize = ROWS> {
    pub(crate) data: [[T; COLS]; ROWS],
}

impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    /// Creates a new instance of the matrix with default values.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix: Matrix<i32, 2, 2> = Matrix::new();
    /// assert_eq!(matrix[(0, 0)], 0);
    /// assert_eq!(matrix[(0, 1)], 0);
    /// assert_eq!(matrix[(1, 0)], 0);
    /// assert_eq!(matrix[(1, 1)], 0);
    /// ```
    pub fn new() -> Self {
        Self {
            data: [[T::zero(); COLS]; ROWS],
        }
    }

    /// Creates a matrix from a 2D array.
    ///
    /// # Arguments
    ///
    /// * `data` - A 2D array containing the matrix elements.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let data = [[1, 2], [3, 4]];
    /// let matrix = Matrix::from_array(data);
    /// assert_eq!(matrix[(0, 0)], 1);
    /// assert_eq!(matrix[(0, 1)], 2);
    /// assert_eq!(matrix[(1, 0)], 3);
    /// assert_eq!(matrix[(1, 1)], 4);
    /// ```
    pub fn from_array(data: [[T; COLS]; ROWS]) -> Self {
        Self { data }
    }

    /// Draws a visual representation of the matrix.
    ///
    /// # Arguments
    ///
    /// * `w` - A character to represent the path in the matrix.
    /// * `b` - A character to represent the background in the matrix.
    ///
    /// # Returns
    ///
    /// A `MatrixVis` object representing the visual representation of the matrix.
    pub fn draw(&self, w: char, b: char) -> MatrixVis<ROWS, COLS> {
        MatrixVis::<ROWS, COLS>::from_mat(self, w, b)
    }

    /// Returns the elements of the specified row.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the row to retrieve.
    ///
    /// # Returns
    ///
    /// An array containing the elements of the specified row.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix = Matrix::from_array([[1, 2], [3, 4]]);
    /// let row = matrix.row(1);
    /// assert_eq!(row, [3, 4]);
    /// ```
    pub fn row(&self, index: usize) -> [T; COLS] {
        self.data[index]
    }

    /// Returns the elements of the specified column.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the column to retrieve.
    ///
    /// # Returns
    ///
    /// An array containing the elements of the specified column.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix = Matrix::from_array([[1, 2], [3, 4]]);
    /// let col = matrix.col(1);
    /// assert_eq!(col, [2, 4]);
    /// ```
    pub fn col(&self, index: usize) -> [T; ROWS] {
        let mut result = [T::zero(); ROWS];
        for i in 0..ROWS {
            result[i] = self[(i, index)];
        }
        result
    }

    /// Checks if the matrix is square.
    ///
    /// # Returns
    ///
    /// `true` if the matrix is square (i.e., the number of rows is equal to the number of columns),
    /// `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix = Matrix::from_array([[1, 2], [3, 4]]);
    /// assert!(matrix.is_square());
    /// ```
    pub fn is_square(&self) -> bool {
        ROWS == COLS
    }

    /// Checks if the matrix is an identity matrix.
    ///
    /// # Returns
    ///
    /// `true` if the matrix is an identity matrix, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix = Matrix::from_array([[1, 0], [0, 1]]);
    /// assert!(matrix.is_identity());
    /// ```
    pub fn is_identity(&self) -> bool {
        if self.is_square() == false {
            return false;
        }
        for i in 0..ROWS {
            for j in 0..COLS {
                if i == j && self[(i, j)] != T::one() {
                    return false;
                } else if i != j && self[(i, j)] != T::zero() {
                    return false;
                }
            }
        }
        true
    }

    /// Checks if the matrix is diagonal.
    ///
    /// # Returns
    ///
    /// `true` if the matrix is diagonal, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix = Matrix::from_array([[3, 0], [0, 5]]);
    /// assert!(matrix.is_diagonal());
    /// ```
    pub fn is_diagonal(&self) -> bool {
        if self.is_square() == false {
            return false;
        }
        for i in 0..ROWS {
            for j in 0..COLS {
                if i == j && self[(i, j)] == T::zero() {
                    return false;
                } else if i != j && self[(i, j)] != T::zero() {
                    return false;
                }
            }
        }
        true
    }

    /// Checks if the matrix is symmetric.
    ///
    /// # Returns
    ///
    /// `true` if the matrix is symmetric, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix = Matrix::from_array([[1, 2], [2, 1]]);
    /// assert!(matrix.is_symmetric());
    /// ```
    pub fn is_symmetric(&self) -> bool {
        if self.is_square() == false {
            return false;
        }
        for i in 0..ROWS {
            for j in 0..COLS {
                if self[(i, j)] != self[(j, i)] {
                    return false;
                }
            }
        }
        true
    }

    /// Checks if the matrix is upper triangular.
    ///
    /// # Returns
    ///
    /// `true` if the matrix is upper triangular, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix = Matrix::from_array([[1, 2], [0, 3]]);
    /// assert!(matrix.is_upper_triangular());
    /// ```
    pub fn is_upper_triangular(&self) -> bool {
        if self.is_square() == false {
            return false;
        }
        for i in 0..ROWS {
            for j in 0..i {
                if self[(i, j)] != T::zero() {
                    return false;
                }
            }
        }
        true
    }

    /// Checks if the matrix is lower triangular.
    ///
    /// # Returns
    ///
    /// `true` if the matrix is lower triangular, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix = Matrix::from_array([[1, 0], [2, 3]]);
    /// assert!(matrix.is_lower_triangular());
    /// ```
    pub fn is_lower_triangular(&self) -> bool {
        if self.is_square() == false {
            return false;
        }
        for i in 0..ROWS {
            for j in i + 1..COLS {
                if self[(i, j)] != T::zero() {
                    return false;
                }
            }
        }
        true
    }
}

impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> std::fmt::Display
    for Matrix<T, ROWS, COLS>
{
    /// Formats the matrix for display.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix = Matrix::from_array([[1, 2], [3, 4]]);
    /// println!("{}", matrix);
    /// // Output:
    /// //  1  2
    /// //  3  4
    /// assert_eq!(format!("{}", matrix), "\n 1 2\n 3 4\n");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        let mut lengths = HashMap::<usize, usize>::new();
        for j in 0..COLS {
            for i in 0..ROWS {
                let length = self[(i, j)].to_string().len();
                lengths
                    .entry(j)
                    .and_modify(|e| *e = (*e).max(length))
                    .or_insert(length);
            }
        }

        writeln!(f).expect("Failed to write to formatter");
        for i in 0..ROWS {
            for j in 0..COLS {
                write!(f, "{:width$}", self[(i, j)], width = lengths[&j] + 1)
                    .expect("Failed to write to formatter");
            }
            writeln!(f).expect("Failed to write to formatter");
        }
        Ok(())
    }
}
