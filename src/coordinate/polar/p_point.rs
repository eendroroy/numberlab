use crate::coordinate::cartesian::{CPoint, CPointTrait};
use crate::coordinate::polar::PPointTrait;
use std::fmt::Display;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PPoint {
    pub r: f64,
    pub theta: f64,
}

impl PPointTrait for PPoint {
    fn to_cartesian(&self) -> CPoint {
        let x = self.r * self.theta.cos();
        let y = self.r * self.theta.sin();
        CPoint { x, y }
    }

    fn distance(&self, other: &Self) -> f64 {
        self.distance_squared(other).sqrt()
    }

    fn distance_squared(&self, other: &Self) -> f64 {
        self.r.powf(2.0) + other.r.powf(2.0)
            - 2.0 * self.r * other.r * (self.theta - other.theta).cos()
    }

    fn midpoint(&self, other: &Self) -> Self {
        self.to_cartesian()
            .midpoint(&other.to_cartesian())
            .to_polar()
    }

    fn is_origin(&self) -> bool {
        self.r == 0.0
    }

    fn is_on_x_axis(&self) -> bool {
        self.theta == 0.0 || self.theta == std::f64::consts::PI
    }

    fn is_on_y_axis(&self) -> bool {
        self.theta == std::f64::consts::FRAC_PI_2 || self.theta == std::f64::consts::FRAC_PI_2 * 3.0
    }

    fn slope(&self, other: &Self) -> f64 {
        self.to_cartesian().slope(&other.to_cartesian())
    }
}

impl Display for PPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{ r: {}, θ: {:} }}", self.r, self.theta)
    }
}
