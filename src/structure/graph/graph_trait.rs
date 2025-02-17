use crate::one::One;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

pub trait GraphDistanceTrait:
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

impl<T> GraphDistanceTrait for T where
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

// pub trait GraphDistanceTraitFraction: GraphDistanceTrait + From<f32> + Into<f64> {}
//
// impl<T> GraphDistanceTraitFraction for T where T: GraphDistanceTrait + From<f32> + Into<f64> {}
