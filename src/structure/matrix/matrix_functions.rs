use crate::one::One;
use crate::structure::matrix::{Matrix, MatrixTrait};
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

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
/// use numberlab::structure::matrix::{Matrix, MatrixTrait, identity};
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
pub fn identity<T, const SIZE: usize>() -> Matrix<T, SIZE, SIZE>
where
    T: Default
    + One
    + Copy
    + Add<Output=T>
    + Sub<Output=T>
    + Mul<Output=T>
    + Div<Output=T>
    + Display,
{
    let mut result = Matrix::<T, SIZE, SIZE>::new();
    for i in 0..SIZE {
        for j in 0..SIZE {
            if i == j {
                result[(i, j)] = T::one();
            }
        }
    }
    result
}

/// Converts the given matrix to an upper triangular matrix.
///
/// An upper triangular matrix is a matrix in which all the elements below the main diagonal are zero.
///
/// # Parameters
///
/// * `matrix` - A mutable reference to the matrix to be converted.
///
/// # Type Parameters
///
/// * `T` - The type of the elements in the matrix.
/// * `SIZE` - The number of rows and columns in the matrix.
///
/// # Returns
///
/// A mutable reference to the upper triangular matrix.
///
/// # Example
///
/// ```
/// use numberlab::structure::matrix::{Matrix, MatrixTrait, upper_triangular};
///
/// let mut matrix = Matrix::from_array([
///     [1, 2, 3],
///     [4, 5, 6],
///     [7, 8, 9]
/// ]);
///
/// let upper = upper_triangular(&mut matrix);
///
/// assert_eq!(upper[(0, 0)], 1);
/// assert_eq!(upper[(0, 1)], 2);
/// assert_eq!(upper[(0, 2)], 3);
/// assert_eq!(upper[(1, 1)], 5);
/// assert_eq!(upper[(1, 2)], 6);
/// assert_eq!(upper[(2, 2)], 9);
/// assert_eq!(upper[(1, 0)], 0);
/// assert_eq!(upper[(2, 0)], 0);
/// assert_eq!(upper[(2, 1)], 0);
/// ```
pub fn upper_triangular<T, const SIZE: usize>(
    matrix: &mut Matrix<T, SIZE, SIZE>,
) -> &mut Matrix<T, SIZE, SIZE>
where
    T: Default
    + One
    + Copy
    + Add<Output=T>
    + Sub<Output=T>
    + Mul<Output=T>
    + Div<Output=T>
    + Display,
{
    for j in 0..SIZE {
        for i in (j + 1)..SIZE {
            matrix[(i, j)] = T::default();
        }
    }
    matrix
}

/// Converts the given matrix to a lower triangular matrix.
///
/// A lower triangular matrix is a matrix in which all the elements above the main diagonal are zero.
///
/// # Parameters
///
/// * `matrix` - A mutable reference to the matrix to be converted.
///
/// # Type Parameters
///
/// * `T` - The type of the elements in the matrix.
/// * `SIZE` - The number of rows and columns in the matrix.
///
/// # Returns
///
/// A mutable reference to the lower triangular matrix.
///
/// # Example
///
/// ```
/// use numberlab::structure::matrix::{Matrix, MatrixTrait, lower_triangular};
///
/// let mut matrix = Matrix::from_array([
///     [1, 2, 3],
///     [4, 5, 6],
///     [7, 8, 9]
/// ]);
///
/// let lower = lower_triangular(&mut matrix);
///
/// assert_eq!(lower[(0, 0)], 1);
/// assert_eq!(lower[(1, 0)], 4);
/// assert_eq!(lower[(2, 0)], 7);
/// assert_eq!(lower[(1, 1)], 5);
/// assert_eq!(lower[(2, 1)], 8);
/// assert_eq!(lower[(2, 2)], 9);
/// assert_eq!(lower[(0, 1)], 0);
/// assert_eq!(lower[(0, 2)], 0);
/// assert_eq!(lower[(1, 2)], 0);
/// ```
pub fn lower_triangular<T, const SIZE: usize>(
    matrix: &mut Matrix<T, SIZE, SIZE>,
) -> &mut Matrix<T, SIZE, SIZE>
where
    T: Default
    + One
    + Copy
    + Add<Output=T>
    + Sub<Output=T>
    + Mul<Output=T>
    + Div<Output=T>
    + Display,
{
    for j in 0..SIZE {
        for i in 0..j {
            matrix[(i, j)] = T::default();
        }
    }
    matrix
}
