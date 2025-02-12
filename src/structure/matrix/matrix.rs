use crate::one::One;
use crate::structure::matrix::matrix_trait::MatrixTrait;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

/// Macro to create a `Matrix` from a nested array.
///
/// # Example
///
/// ```
/// use numberlab::mat;
/// use numberlab::structure::matrix::Matrix;
/// use numberlab::structure::matrix::MatrixTrait;
///
/// let matrix = mat![
///     [1, 2, 3],
///     [4, 5, 6],
///     [7, 8, 9]
/// ];
///
/// assert_eq!(matrix, mat![[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
/// assert_eq!(format!("{}", matrix), "\n 1 2 3\n 4 5 6\n 7 8 9\n");
/// ```
#[macro_export]
macro_rules! mat {
    ($([$($elem:expr),* $(,)?]),* $(,)?) => {
        {
            let data = [$( [ $($elem),* ] ),*];
            Matrix::from_array(data)
        }
    };
}

/// A generic Matrix struct that holds a 2D array of elements.
///
/// # Type Parameters
///
/// * `T` - The type of the elements in the matrix.
/// * `ROWS` - The number of rows in the matrix.
/// * `COLS` - The number of columns in the matrix.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

impl<T, const ROWS: usize, const COLS: usize> MatrixTrait<T, ROWS, COLS> for Matrix<T, ROWS, COLS>
where
    T: Default
        + One
        + Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Display,
{
    type Transpose = Matrix<T, COLS, ROWS>;

    fn new() -> Self {
        Self {
            data: [[T::default(); COLS]; ROWS],
        }
    }

    fn from_array(data: [[T; COLS]; ROWS]) -> Self {
        Self { data }
    }

    fn identity() -> Self {
        let mut result = Self::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                if i == j {
                    result[(i, j)] = T::one();
                }
            }
        }
        result
    }

    fn transpose(&self) -> Matrix<T, COLS, ROWS> {
        let mut result = Matrix::<T, COLS, ROWS>::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                result[(j, i)] = self[(i, j)];
            }
        }
        result
    }
}

impl<T, const ROWS: usize, const COLS: usize> Index<(usize, usize)> for Matrix<T, ROWS, COLS> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<T, const ROWS: usize, const COLS: usize> IndexMut<(usize, usize)> for Matrix<T, ROWS, COLS> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

impl<T, const ROWS: usize, const COLS: usize> Add for Matrix<T, ROWS, COLS>
where
    T: Default
        + One
        + Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Display,
{
    type Output = Self;

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

impl<T, const ROWS: usize, const COLS: usize> Sub for Matrix<T, ROWS, COLS>
where
    T: Default
        + One
        + Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Display,
{
    type Output = Self;

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

impl<T, const R: usize, const C: usize, const K: usize> Mul<Matrix<T, C, K>> for Matrix<T, R, C>
where
    T: Default
        + One
        + Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Display,
{
    type Output = Matrix<T, R, K>;

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

impl<T, const ROWS: usize, const COLS: usize> Div for Matrix<T, ROWS, COLS>
where
    T: Default
        + One
        + Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Display,
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                result[(i, j)] = self[(i, j)] / other[(i, j)];
            }
        }
        result
    }
}

impl<T, const ROWS: usize, const COLS: usize> Display for Matrix<T, ROWS, COLS>
where
    T: Default
        + One
        + Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Display,
{
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

        writeln!(f)?;
        for i in 0..ROWS {
            for j in 0..COLS {
                write!(f, "{:width$}", self[(i, j)], width = lengths[&j] + 1)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
