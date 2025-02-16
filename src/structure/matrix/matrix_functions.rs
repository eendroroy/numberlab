use crate::structure::matrix::matrix_trait::MatrixDataTrait;
use crate::structure::matrix::Matrix;

/// Creates an identity matrix of the given size.
///
/// An identity matrix is a square matrix with ones on the main diagonal and zeros elsewhere.
///
/// # Type Parameters
///
/// * `T` - The type of the elements in the matrix.
/// * `SIZE` - The number of rows and columns in the matrix.
///
/// # Returns
///
/// An identity matrix of the given size.
///
/// # Example
///
/// ```
/// use numberlab::structure::matrix::{Matrix, identity};
///
/// let matrix = identity::<i32, 3>();
///
/// assert_eq!(matrix[(0, 0)], 1);
/// assert_eq!(matrix[(1, 1)], 1);
/// assert_eq!(matrix[(2, 2)], 1);
/// assert_eq!(matrix[(0, 1)], 0);
/// assert_eq!(matrix[(0, 2)], 0);
/// assert_eq!(matrix[(1, 0)], 0);
/// assert_eq!(matrix[(1, 2)], 0);
/// assert_eq!(matrix[(2, 0)], 0);
/// assert_eq!(matrix[(2, 1)], 0);
/// ```
pub fn identity<T: MatrixDataTrait, const SIZE: usize>() -> Matrix<T, SIZE, SIZE> {
    let mut result = Matrix::<T, SIZE, SIZE>::new();
    (0..SIZE).for_each(|i| result[(i, i)] = T::one());
    result
}

/// Transposes the given matrix.
///
/// # Type Parameters
///
/// * `T` - The type of the elements in the matrix.
/// * `ROWS` - The number of rows in the matrix.
/// * `COLS` - The number of columns in the matrix.
///
/// # Arguments
///
/// * `matrix` - The matrix to be transposed.
///
/// # Returns
///
/// A new matrix that is the transpose of the given matrix.
///
/// # Example
///
/// ```
/// use numberlab::structure::matrix::{Matrix, transpose};
///
/// let matrix = Matrix::<i32, 2, 3>::from_array([[1, 2, 3], [4, 5, 6]]);
/// let transposed = transpose(matrix);
///
/// assert_eq!(transposed[(0, 0)], 1);
/// assert_eq!(transposed[(1, 0)], 2);
/// assert_eq!(transposed[(2, 0)], 3);
/// assert_eq!(transposed[(0, 1)], 4);
/// assert_eq!(transposed[(1, 1)], 5);
/// assert_eq!(transposed[(2, 1)], 6);
/// ```
pub fn transpose<T: MatrixDataTrait, const ROWS: usize, const COLS: usize>(
    matrix: Matrix<T, ROWS, COLS>,
) -> Matrix<T, COLS, ROWS> {
    let mut result = Matrix::<T, COLS, ROWS>::new();
    for i in 0..ROWS {
        for j in 0..COLS {
            result[(j, i)] = matrix[(i, j)];
        }
    }
    result
}

/// Converts the given matrix to its upper triangular form and returns it.
///
/// # Type Parameters
///
/// * `T` - The type of the elements in the matrix.
/// * `SIZE` - The number of rows and columns in the matrix.
///
/// # Arguments
///
/// * `matrix` - The matrix to be converted.
///
/// # Returns
///
/// A new matrix that is the upper triangular form of the given matrix.
///
/// # Example
///
/// ```
/// use numberlab::structure::matrix::{Matrix, upper_triangular};
///
/// let matrix = Matrix::<i32, 3, 3>::from_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
/// let upper = upper_triangular(matrix);
///
/// assert_eq!(upper, Matrix::<i32, 3, 3>::from_array([[1, 2, 3], [0, 5, 6], [0, 0, 9]]));
/// ```
pub fn upper_triangular<T: MatrixDataTrait, const SIZE: usize>(
    matrix: Matrix<T, SIZE, SIZE>,
) -> Matrix<T, SIZE, SIZE> {
    let mut result = matrix.clone();
    for j in 0..SIZE {
        for i in (j + 1)..SIZE {
            result[(i, j)] = T::default();
        }
    }
    result
}

/// Converts the given matrix to its lower triangular form and returns it.
///
/// # Type Parameters
///
/// * `T` - The type of the elements in the matrix.
/// * `SIZE` - The number of rows and columns in the matrix.
///
/// # Arguments
///
/// * `matrix` - The matrix to be converted.
///
/// # Returns
///
/// A new matrix that is the lower triangular form of the given matrix.
///
/// # Example
///
/// ```
/// use numberlab::structure::matrix::{Matrix, lower_triangular};
///
/// let matrix = Matrix::<i32, 3, 3>::from_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
/// let lower = lower_triangular(matrix);
///
/// assert_eq!(lower, Matrix::<i32, 3, 3>::from_array([[1, 0, 0], [4, 5, 0], [7, 8, 9]]));
/// ```
pub fn lower_triangular<T: MatrixDataTrait, const SIZE: usize>(
    matrix: Matrix<T, SIZE, SIZE>,
) -> Matrix<T, SIZE, SIZE> {
    let mut result = matrix.clone();
    for i in 0..SIZE {
        for j in i + 1..SIZE {
            result[(i, j)] = T::default();
        }
    }
    result
}
