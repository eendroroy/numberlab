use crate::coordinate::cartesian::Point1DTrait;
use std::fmt::Display;

/// A structure representing a point in one-dimensional space.
///
/// # Fields
///
/// * `x` - The x-coordinate of the point.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point1D {
    pub x: f64,
}

impl Point1DTrait for Point1D {
    fn distance(&self, other: &Self) -> f64 {
        (self.x - other.x).abs()
    }

    fn position(&self, velocity: f64, time: f64) -> Self {
        Point1D {
            x: self.x + velocity * time,
        }
    }
}

impl Display for Point1D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{ x: {} }}", self.x)
    }
}
