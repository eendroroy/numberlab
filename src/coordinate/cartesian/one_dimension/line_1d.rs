use crate::coordinate::cartesian::one_dimension::Point1D;
use crate::coordinate::cartesian::{Line1DTrait, Point1DTrait};
use std::fmt::Display;

/// A structure representing a line in one-dimensional space.
///
/// # Fields
///
/// * `start` - The starting point of the line.
/// * `end` - The ending point of the line.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line1D {
    pub start: Point1D,
    pub end: Point1D,
}

impl Line1DTrait<Point1D> for Line1D {
    fn length(&self) -> f64 {
        self.start.distance(&self.end)
    }

    fn midpoint(&self) -> Point1D {
        Point1D {
            x: (self.start.x + self.end.x) / 2.0,
        }
    }
}

impl Display for Line1D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{ start: {}, end: {} }}", self.start, self.end)
    }
}
