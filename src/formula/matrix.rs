use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

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
impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS>
where
    T: Default + Copy,
{
    /// Creates a new matrix with default values.
    ///
    /// # Returns
    ///
    /// A new `Matrix` instance with all elements set to their default values.
    pub fn new() -> Self {
        Self {
            data: [[T::default(); COLS]; ROWS],
        }
    }

    /// Creates a new matrix from a given 2D array.
    ///
    /// # Parameters
    ///
    /// * `data` - A 2D array containing the elements to initialize the matrix with.
    ///
    /// # Returns
    ///
    /// A new `Matrix` instance initialized with the provided data.
    pub fn from_array(data: [[T; COLS]; ROWS]) -> Self {
        Self { data }
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
    T: Add<Output=T> + Default + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }
}

impl<T, const ROWS: usize, const COLS: usize> Sub for Matrix<T, ROWS, COLS>
where
    T: Sub<Output=T> + Default + Copy,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        result
    }
}

impl<T, const R: usize, const C: usize, const K: usize> Mul<Matrix<T, C, K>> for Matrix<T, R, C>
where
    T: Mul<Output=T> + Add<Output=T> + Default + Copy,
{
    type Output = Matrix<T, R, K>;

    fn mul(self, other: Matrix<T, C, K>) -> Self::Output {
        let mut result = Matrix::<T, R, K>::new();
        for i in 0..R {
            for j in 0..K {
                for k in 0..C {
                    result.data[i][j] = result.data[i][j] + (self.data[i][k] * other.data[k][j]);
                }
            }
        }
        result
    }
}

impl<T, const ROWS: usize, const COLS: usize> Div for Matrix<T, ROWS, COLS>
where
    T: Div<Output=T> + Default + Copy,
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[i][j] = self.data[i][j] / other.data[i][j];
            }
        }
        result
    }
}

impl<T, const ROWS: usize, const COLS: usize> Display for Matrix<T, ROWS, COLS>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut lengths = HashMap::<usize, usize>::new();
        for j in 0..COLS {
            for i in 0..ROWS {
                let length = self.data[i][j].to_string().len();
                lengths
                    .entry(j)
                    .and_modify(|e| *e = (*e).max(length))
                    .or_insert(length);
            }
        }

        writeln!(f)?;
        for i in 0..ROWS {
            for j in 0..COLS {
                write!(f, "{:width$}", self.data[i][j], width = lengths[&j] + 1)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
