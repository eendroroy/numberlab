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
