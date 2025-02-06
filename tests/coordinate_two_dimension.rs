use numberlab::coordinate::cartesian::{Point2D, Point2DTrait};

#[test]
fn should_display_point() {
    let p = Point2D { x: 1.0, y: 2.0 };

    assert_eq!(format!("{}", p), "{ x: 1, y: 2 }");
}

#[test]
fn should_calculate_distance() {
    let p1 = Point2D { x: 1.0, y: 2.0 };
    let p2 = Point2D { x: 3.0, y: 4.0 };

    assert_eq!(p1.distance(&p1), 0.0);
    assert_eq!(p1.distance(&p2), 2.8284271247461903);
    assert_eq!(p2.distance(&p1), 2.8284271247461903);
    assert_eq!(p2.distance(&p2), 0.0);
}

#[test]
fn should_calculate_distance_squared() {
    let p1 = Point2D { x: 1.0, y: 2.0 };
    let p2 = Point2D { x: 3.0, y: 4.0 };

    assert_eq!(p1.distance_squared(&p1), 0.0);
    assert_eq!(p1.distance_squared(&p2), 8.0);
    assert_eq!(p2.distance_squared(&p1), 8.0);
    assert_eq!(p2.distance_squared(&p2), 0.0);
}

#[test]
fn should_calculate_midpoint() {
    let p1 = Point2D { x: 1.0, y: 2.0 };
    let p2 = Point2D { x: 3.0, y: 4.0 };

    assert_eq!(p1.midpoint(&p1).x, 1.0);
    assert_eq!(p1.midpoint(&p1).y, 2.0);
    assert_eq!(p1.midpoint(&p2).x, 2.0);
    assert_eq!(p1.midpoint(&p2).y, 3.0);
    assert_eq!(p2.midpoint(&p1).x, 2.0);
    assert_eq!(p2.midpoint(&p1).y, 3.0);
    assert_eq!(p2.midpoint(&p2).x, 3.0);
    assert_eq!(p2.midpoint(&p2).y, 4.0);
}

#[test]
fn should_calculate_is_origin() {
    let p1 = Point2D { x: 0.0, y: 0.0 };
    let p2 = Point2D { x: 1.0, y: 2.0 };

    assert_eq!(p1.is_origin(), true);
    assert_eq!(p2.is_origin(), false);
}

#[test]
fn should_calculate_is_on_x_axis() {
    let p1 = Point2D { x: 0.0, y: 0.0 };
    let p2 = Point2D { x: 1.0, y: 2.0 };

    assert_eq!(p1.is_on_x_axis(), true);
    assert_eq!(p2.is_on_x_axis(), false);
}

#[test]
fn should_calculate_is_on_y_axis() {
    let p1 = Point2D { x: 0.0, y: 0.0 };
    let p2 = Point2D { x: 1.0, y: 2.0 };

    assert_eq!(p1.is_on_y_axis(), true);
    assert_eq!(p2.is_on_y_axis(), false);
}

#[test]
fn should_calculate_slope() {
    let p1 = Point2D { x: 1.0, y: 2.0 };
    let p2 = Point2D { x: 3.0, y: 4.0 };

    assert_eq!(p1.slope(&p1).is_nan(), true);
    assert_eq!(p1.slope(&p2), 1.0);
    assert_eq!(p2.slope(&p1), 1.0);
    assert_eq!(p2.slope(&p2).is_nan(), true);
}

#[test]
fn should_calculate_slope_intercept() {
    let p1 = Point2D { x: 1.0, y: 2.0 };
    let p2 = Point2D { x: 3.0, y: 4.0 };

    assert_eq!(p1.slope_intercept(&p2), (1.0, 1.0));
    assert_eq!(p2.slope_intercept(&p1), (1.0, 1.0));

    let (m, a) = p1.slope_intercept(&p1);
    assert_eq!(m.is_nan(), true);
    assert_eq!(a.is_nan(), true);

    let (m, a) = p2.slope_intercept(&p2);
    assert_eq!(m.is_nan(), true);
    assert_eq!(a.is_nan(), true);
}
