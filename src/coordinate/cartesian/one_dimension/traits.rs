/// A trait representing a one-dimensional point with various geometric operations.
pub trait Point1DTrait {
    /// Calculates the distance between two points.
    ///
    /// # Arguments
    ///
    /// * `other` - Another point to calculate the distance to.
    ///
    /// # Returns
    ///
    /// A `f64` representing the distance between the two points.
    fn distance(&self, other: &Self) -> f64;

    /// Calculates the position of the point after moving with a given velocity for a given time.
    ///
    /// # Arguments
    ///
    /// * `velocity` - The velocity at which the point is moving.
    /// * `time` - The time for which the point has been moving.
    ///
    /// # Returns
    ///
    /// A new instance of `Self` representing the new position of the point.
    fn position(&self, velocity: f64, time: f64) -> Self;
}

/// A trait representing a one-dimensional line with various geometric operations.
pub trait Line1DTrait<PointType: Point1DTrait> {
    /// Calculates the length of the line.
    ///
    /// # Returns
    ///
    /// A `f64` representing the length of the line.
    fn length(&self) -> f64;

    /// Calculates the midpoint of the line.
    ///
    /// # Returns
    ///
    /// A `PointType` representing the midpoint of the line.
    fn midpoint(&self) -> PointType;
}
