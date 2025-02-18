use crate::one::One;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

pub trait GraphWeightTrait:
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

impl<W> GraphWeightTrait for W where
    W: One
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

// pub trait GraphDistanceTraitFraction: GraphDistanceTrait + From<f32> + Into<f64> {}
//
// impl<T> GraphDistanceTraitFraction for T where T: GraphDistanceTrait + From<f32> + Into<f64> {}
