use crate::coordinate::cartesian::CPointTrait;
use crate::coordinate::polar::PPoint;
use std::fmt::Display;

/// A structure representing a point in tow-dimensional space.
///
/// # Fields
///
/// * `x` - The x-coordinate of the point.
/// * `y` - The y-coordinate of the point.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CPoint {
    pub x: f64,
    pub y: f64,
}

impl CPointTrait for CPoint {
    fn to_polar(&self) -> PPoint {
        let r = self.distance(&CPoint { x: 0.0, y: 0.0 });
        let t = self.y.atan2(self.x);
        PPoint { r, theta: t }
    }

    fn distance(&self, other: &Self) -> f64 {
        self.distance_squared(other).sqrt()
    }

    fn distance_squared(&self, other: &Self) -> f64 {
        (self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)
    }

    fn midpoint(&self, other: &Self) -> Self {
        CPoint {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
        }
    }

    fn is_origin(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    fn is_on_x_axis(&self) -> bool {
        self.y == 0.0
    }

    fn is_on_y_axis(&self) -> bool {
        self.x == 0.0
    }

    fn slope(&self, other: &Self) -> f64 {
        (self.y - other.y) / (self.x - other.x)
    }

    fn slope_intercept(&self, other: &Self) -> (f64, f64) {
        let m = self.slope(other);
        let b = self.y - m * self.x;
        (m, b)
    }
}

impl Display for CPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{ x: {}, y: {} }}", self.x, self.y)
    }
}
