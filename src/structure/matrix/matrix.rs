use crate::structure::matrix::matrix_trait::MatrixDataTrait;
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
            data: [[T::default(); COLS]; ROWS],
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
        let mut result = [T::default(); ROWS];
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
                } else if i != j && self[(i, j)] != T::default() {
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
                if i == j && self[(i, j)] == T::default() {
                    return false;
                } else if i != j && self[(i, j)] != T::default() {
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
                if self[(i, j)] != T::default() {
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
                if self[(i, j)] != T::default() {
                    return false;
                }
            }
        }
        true
    }

    /// Returns the transpose of the matrix.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix = Matrix::from_array([[1, 2, 3], [4, 5, 6]]);
    /// let transposed = matrix.transpose();
    /// assert_eq!(transposed, Matrix::from_array([[1, 4], [2, 5], [3, 6]]));
    /// ```
    pub fn transpose(&self) -> Matrix<T, COLS, ROWS> {
        let mut result = Matrix::<T, COLS, ROWS>::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                result[(j, i)] = self[(i, j)];
            }
        }
        result
    }

    /// Converts the matrix to an upper triangular matrix in place.
    ///
    /// This method sets all the elements below the main diagonal to the default value.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let mut matrix = Matrix::from_array([[1, 2], [3, 4]]);
    /// matrix.upper_triangular();
    /// assert_eq!(matrix, Matrix::from_array([[1, 2], [0, 4]]));
    /// ```
    pub fn upper_triangular(&mut self) {
        if self.is_square() == false {
            panic!("Matrix [{} x {}] is not square", ROWS, COLS);
        }
        for j in 0..ROWS {
            for i in (j + 1)..COLS {
                self[(i, j)] = T::default();
            }
        }
    }

    /// Converts the matrix to an upper triangular matrix and returns the result.
    ///
    /// This method creates a new matrix that is an upper triangular version of the original matrix.
    ///
    /// # Returns
    ///
    /// A new matrix that is the upper triangular version of the original matrix.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix = Matrix::from_array([[1, 2], [3, 4]]);
    /// let upper_triangular = matrix.to_upper_triangular();
    /// assert_eq!(upper_triangular, Matrix::from_array([[1, 2], [0, 4]]));
    /// ```
    pub fn to_upper_triangular(&self) -> Self {
        if self.is_square() == false {
            panic!("Matrix [{} x {}] is not square", ROWS, COLS);
        }
        let mut result = self.clone();
        result.upper_triangular();
        result
    }
    /// Converts the matrix to a lower triangular matrix in place.
    ///
    /// This method sets all the elements above the main diagonal to the default value.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let mut matrix = Matrix::from_array([[1, 2], [3, 4]]);
    /// matrix.lower_triangular();
    /// assert_eq!(matrix, Matrix::from_array([[1, 0], [3, 4]]));
    /// ```
    pub fn lower_triangular(&mut self) {
        if self.is_square() == false {
            panic!("Matrix [{} x {}] is not square", ROWS, COLS);
        }
        for i in 0..ROWS {
            for j in i + 1..COLS {
                self[(i, j)] = T::default();
            }
        }
    }

    /// Converts the matrix to a lower triangular matrix and returns the result.
    ///
    /// This method creates a new matrix that is a lower triangular version of the original matrix.
    ///
    /// # Returns
    ///
    /// A new matrix that is the lower triangular version of the original matrix.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix = Matrix::from_array([[1, 2], [3, 4]]);
    /// let lower_triangular = matrix.to_lower_triangular();
    /// assert_eq!(lower_triangular, Matrix::from_array([[1, 0], [3, 4]]));
    /// ```
    pub fn to_lower_triangular(&self) -> Self {
        if self.is_square() == false {
            panic!("Matrix [{} x {}] is not square", ROWS, COLS);
        }
        let mut result = self.clone();
        result.lower_triangular();
        result
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
