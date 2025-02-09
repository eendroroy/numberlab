use crate::coordinate::polar::PPoint;

/// A trait representing a 2D point with various geometric operations.
pub trait CPointTrait {
    /// Converts the Cartesian point to a polar point.
    ///
    /// # Returns
    ///
    /// A polar point representation of the Cartesian point.
    fn to_polar(&self) -> PPoint;

    /// Calculates the distance between two points.
    ///
    /// # Arguments
    ///
    /// * `other` - Another point to calculate the distance to.
    ///
    /// # Returns
    ///
    /// The distance between the two points.
    fn distance(&self, other: &Self) -> f64;

    /// Calculates the squared distance between two points.
    ///
    /// # Arguments
    ///
    /// * `other` - Another point to calculate the squared distance to.
    ///
    /// # Returns
    ///
    /// The squared distance between the two points.
    fn distance_squared(&self, other: &Self) -> f64;

    /// Calculates the midpoint between two points.
    ///
    /// # Arguments
    ///
    /// * `other` - Another point to calculate the midpoint with.
    ///
    /// # Returns
    ///
    /// The midpoint between the two points.
    fn midpoint(&self, other: &Self) -> Self;

    /// Checks if the point is at the origin (0, 0).
    ///
    /// # Returns
    ///
    /// `true` if the point is at the origin, `false` otherwise.
    fn is_origin(&self) -> bool;

    /// Checks if the point is on the x-axis.
    ///
    /// # Returns
    ///
    /// `true` if the point is on the x-axis, `false` otherwise.
    fn is_on_x_axis(&self) -> bool;

    /// Checks if the point is on the y-axis.
    ///
    /// # Returns
    ///
    /// `true` if the point is on the y-axis, `false` otherwise.
    fn is_on_y_axis(&self) -> bool;

    /// Calculates the slope between two points.
    ///
    /// # Arguments
    ///
    /// * `other` - Another point to calculate the slope with.
    ///
    /// # Returns
    ///
    /// The slope between the two points.
    fn slope(&self, other: &Self) -> f64;

    /// Calculates the slope and y-intercept of the line passing through two points.
    ///
    /// # Arguments
    ///
    /// * `other` - Another point to calculate the slope and intercept with.
    ///
    /// # Returns
    ///
    /// A tuple containing the slope and y-intercept of the line.
    fn slope_intercept(&self, other: &Self) -> (f64, f64);
}
