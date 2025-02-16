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
