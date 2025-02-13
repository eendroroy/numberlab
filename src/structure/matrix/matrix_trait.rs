use crate::structure::matrix::one::One;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

pub trait MatrixDataTrait:
    Default
    + One
    + Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Display
{
}

impl<T> MatrixDataTrait for T where
    T: Default
        + One
        + Copy
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Display
{
}

pub trait MatrixDataTraitFraction: MatrixDataTrait + From<f32> + Into<f64> {}

impl<T> MatrixDataTraitFraction for T where T: MatrixDataTrait + From<f32> + Into<f64> {}

/// A trait representing a matrix with generic element type `T` and fixed dimensions `ROWS` x `COLS`.
pub trait MatrixTrait<T, const ROWS: usize, const COLS: usize>
where
    T: MatrixDataTrait,
{
    type Transpose;

    /// Creates a new instance of the matrix.
    fn new() -> Self;

    /// Creates a matrix from a 2D array.
    ///
    /// # Arguments
    ///
    /// * `data` - A 2D array containing the matrix elements.
    fn from_array(data: [[T; COLS]; ROWS]) -> Self;

    /// Returns the transpose of the matrix.
    ///
    /// # Returns
    ///
    /// A new matrix which is the transpose of the original matrix.
    fn transpose(&self) -> Self::Transpose;

    /// Returns the elements of the specified row.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the row to retrieve.
    ///
    /// # Returns
    ///
    /// An array containing the elements of the specified row.
    fn row(&self, index: usize) -> [T; COLS];

    /// Returns the elements of the specified column.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the column to retrieve.
    ///
    /// # Returns
    ///
    /// An array containing the elements of the specified column.
    fn col(&self, index: usize) -> [T; ROWS];
}
