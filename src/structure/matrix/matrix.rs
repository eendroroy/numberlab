use crate::structure::matrix::matrix_trait::{MatrixDataTrait, MatrixDataTraitFraction};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

/// A generic Matrix struct that holds a 2D array of elements.
///
/// # Type Parameters
///
/// * `T` - The type of the elements in the matrix.
/// * `ROWS` - The number of rows in the matrix.
/// * `COLS` - The number of columns in the matrix.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize = ROWS> {
    data: [[T; COLS]; ROWS],
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
}

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

impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> Add for Matrix<T, ROWS, COLS> {
    type Output = Self;

    /// Adds two matrices element-wise.
    ///
    /// # Arguments
    ///
    /// * `self` - The first matrix.
    /// * `other` - The second matrix.
    ///
    /// # Returns
    ///
    /// A new matrix which is the result of the element-wise addition of the two matrices.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix1 = Matrix::from_array([[1, 2], [3, 4]]);
    /// let matrix2 = Matrix::from_array([[5, 6], [7, 8]]);
    /// let result = matrix1 + matrix2;
    /// assert_eq!(result, Matrix::from_array([[6, 8], [10, 12]]));
    /// ```
    fn add(self, other: Self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                result[(i, j)] = self[(i, j)] + other[(i, j)];
            }
        }
        result
    }
}

impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> Sub for Matrix<T, ROWS, COLS> {
    type Output = Self;

    /// Subtracts two matrices element-wise.
    ///
    /// # Arguments
    ///
    /// * `self` - The first matrix.
    /// * `other` - The second matrix.
    ///
    /// # Returns
    ///
    /// A new matrix which is the result of the element-wise subtraction of the two matrices.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix1 = Matrix::from_array([[5, 6], [7, 8]]);
    /// let matrix2 = Matrix::from_array([[1, 2], [3, 4]]);
    /// let result = matrix1 - matrix2;
    /// assert_eq!(result, Matrix::from_array([[4, 4], [4, 4]]));
    /// ```
    fn sub(self, other: Self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                result[(i, j)] = self[(i, j)] - other[(i, j)];
            }
        }
        result
    }
}

impl<T: MatrixDataTrait, const R: usize, const C: usize, const K: usize> Mul<Matrix<T, C, K>>
    for Matrix<T, R, C>
{
    type Output = Matrix<T, R, K>;

    /// Multiplies two matrices.
    ///
    /// # Arguments
    ///
    /// * `self` - The first matrix.
    /// * `other` - The second matrix.
    ///
    /// # Returns
    ///
    /// A new matrix which is the result of the matrix multiplication of the two matrices.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix1 = Matrix::from_array([[1, 2], [3, 4]]);
    /// let matrix2 = Matrix::from_array([[5, 6], [7, 8]]);
    /// let result = matrix1 * matrix2;
    /// assert_eq!(result, Matrix::from_array([[19, 22], [43, 50]]));
    /// ```
    fn mul(self, other: Matrix<T, C, K>) -> Self::Output {
        let mut result = Matrix::<T, R, K>::new();
        for i in 0..R {
            for j in 0..K {
                for k in 0..C {
                    result[(i, j)] = result[(i, j)] + (self.data[i][k] * other.data[k][j]);
                }
            }
        }
        result
    }
}

impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> Add<T> for Matrix<T, ROWS, COLS> {
    type Output = Self;

    /// Adds a scalar to each element of the matrix.
    ///
    /// # Arguments
    ///
    /// * `self` - The matrix to which the scalar will be added.
    /// * `other` - The scalar value to add to each element of the matrix.
    ///
    /// # Returns
    ///
    /// A new matrix with the scalar added to each element.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix = Matrix::from_array([[1, 2], [3, 4]]);
    /// let result = matrix + 1;
    /// assert_eq!(result, Matrix::from_array([[2, 3], [4, 5]]));
    /// ```
    fn add(self, other: T) -> Self {
        let mut result = Self::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                result[(i, j)] = self[(i, j)] + other;
            }
        }
        result
    }
}

impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> Sub<T> for Matrix<T, ROWS, COLS> {
    type Output = Self;

    /// Subtracts a scalar from each element of the matrix.
    ///
    /// # Arguments
    ///
    /// * `self` - The matrix from which the scalar will be subtracted.
    /// * `other` - The scalar value to subtract from each element of the matrix.
    ///
    /// # Returns
    ///
    /// A new matrix with the scalar subtracted from each element.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix = Matrix::from_array([[2, 3], [4, 5]]);
    /// let result = matrix - 1;
    /// assert_eq!(result, Matrix::from_array([[1, 2], [3, 4]]));
    /// ```
    fn sub(self, other: T) -> Self {
        let mut result = Self::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                result[(i, j)] = self[(i, j)] - other;
            }
        }
        result
    }
}

impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> Mul<T> for Matrix<T, ROWS, COLS> {
    type Output = Self;

    /// Multiplies each element of the matrix by a scalar.
    ///
    /// # Arguments
    ///
    /// * `self` - The matrix to be multiplied.
    /// * `other` - The scalar value to multiply each element of the matrix by.
    ///
    /// # Returns
    ///
    /// A new matrix with each element multiplied by the scalar.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix = Matrix::from_array([[1, 2], [3, 4]]);
    /// let result = matrix * 2;
    /// assert_eq!(result, Matrix::from_array([[2, 4], [6, 8]]));
    /// ```
    fn mul(self, other: T) -> Self {
        let mut result = Self::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                result[(i, j)] = self[(i, j)] * other;
            }
        }
        result
    }
}

impl<T: MatrixDataTraitFraction, const ROWS: usize, const COLS: usize> Div<T>
    for Matrix<T, ROWS, COLS>
{
    type Output = Self;

    /// Divides each element of the matrix by a scalar.
    ///
    /// # Arguments
    ///
    /// * `self` - The matrix to be divided.
    /// * `other` - The scalar value to divide each element of the matrix by.
    ///
    /// # Returns
    ///
    /// A new matrix with each element divided by the scalar.
    ///
    /// # Example
    ///
    /// ```
    /// use numberlab::structure::matrix::Matrix;
    ///
    /// let matrix = Matrix::from_array([[2.0, 3.0], [6.0, 8.0]]);
    /// let result = matrix / 2.0;
    /// assert_eq!(result, Matrix::from_array([[1.0, 1.5], [3.0, 4.0]]));
    /// ```
    fn div(self, other: T) -> Self {
        let mut result = Self::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                result[(i, j)] = self[(i, j)] / other;
            }
        }
        result
    }
}

impl<T: MatrixDataTrait, const ROWS: usize, const COLS: usize> Display for Matrix<T, ROWS, COLS> {
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
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
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
