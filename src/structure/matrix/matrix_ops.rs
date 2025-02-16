use crate::structure::matrix::matrix_trait::{MatrixDataTrait, MatrixDataTraitFraction};
use crate::structure::matrix::Matrix;
use std::ops::{Add, Div, Mul, Sub};

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
