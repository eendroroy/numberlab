use crate::one::One;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

pub trait MatrixDataTrait:
    One
    + Copy
    + Clone
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialOrd
    + Display
{
}

impl<T> MatrixDataTrait for T where
    T: One
        + Copy
        + Clone
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + PartialOrd
        + Display
{
}

pub trait MatrixDataTraitFraction: MatrixDataTrait + From<f32> + Into<f64> {}

impl<T> MatrixDataTraitFraction for T where T: MatrixDataTrait + From<f32> + Into<f64> {}
