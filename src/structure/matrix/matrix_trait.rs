use crate::one::One;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

/// A trait representing a matrix with generic element type `T` and fixed dimensions `ROWS` x `COLS`.
pub trait MatrixTrait<T, const ROWS: usize, const COLS: usize>
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
}
