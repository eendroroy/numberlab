use numberlab::coordinate::polar::{PPoint, PPointTrait};

#[test]
fn should_display_point() {
    let p = PPoint { r: 1.0, theta: 2.0 };

    assert_eq!(format!("{}", p), "{ r: 1, θ: 2 }");
}

#[test]
fn should_convert_to_cartesian() {
    let p = PPoint { r: 1.0, theta: 2.0 };
    let p_cartesian = p.to_cartesian();

    assert_eq!(p_cartesian.x, -0.4161468365471424);
    assert_eq!(p_cartesian.y, 0.9092974268256817);
}

#[test]
fn should_calculate_distance() {
    let p1 = PPoint { r: 1.0, theta: 2.0 };
    let p2 = PPoint { r: 3.0, theta: 4.0 };

    assert_eq!(p1.distance(&p1), 0.0);
    assert_eq!(p1.distance(&p2), 3.5350927879311533);
    assert_eq!(p2.distance(&p1), 3.5350927879311533);
    assert_eq!(p2.distance(&p2), 0.0);
}

#[test]
fn should_calculate_distance_squared() {
    let p1 = PPoint { r: 1.0, theta: 2.0 };
    let p2 = PPoint { r: 3.0, theta: 4.0 };

    assert_eq!(p1.distance_squared(&p1), 0.0);
    assert_eq!(p1.distance_squared(&p2), 12.496881019282855);
    assert_eq!(p2.distance_squared(&p1), 12.496881019282855);
    assert_eq!(p2.distance_squared(&p2), 0.0);
}

#[test]
fn should_calculate_midpoint() {
    let p1 = PPoint { r: 1.0, theta: 2.0 };
    let p2 = PPoint { r: 3.0, theta: 4.0 };

    assert_eq!(p1.midpoint(&p1).r, 1.0);
    assert_eq!(p1.midpoint(&p1).theta, 2.0);
    assert_eq!(p1.midpoint(&p2).r, 1.3695910868501178);
    assert_eq!(p1.midpoint(&p2).theta, -2.62156537532941);
    assert_eq!(p2.midpoint(&p1).r, 1.3695910868501178);
    assert_eq!(p2.midpoint(&p1).theta, -2.62156537532941);
    assert_eq!(p2.midpoint(&p2).r, 3.0);
    assert_eq!(p2.midpoint(&p2).theta, -2.2831853071795867);
}

#[test]
fn should_calculate_is_origin() {
    let p1 = PPoint { r: 0.0, theta: 0.0 };
    let p2 = PPoint { r: 1.0, theta: 2.0 };

    assert_eq!(p1.is_origin(), true);
    assert_eq!(p2.is_origin(), false);
}

#[test]
fn should_calculate_is_on_x_axis() {
    let p1 = PPoint { r: 0.0, theta: 0.0 };
    let p2 = PPoint { r: 1.0, theta: 2.0 };

    assert_eq!(p1.is_on_x_axis(), true);
    assert_eq!(p2.is_on_x_axis(), false);
}

#[test]
fn should_calculate_is_on_y_axis() {
    let p1 = PPoint { r: 0.0, theta: 0.0 };
    let p2 = PPoint { r: 1.0, theta: 2.0 };

    assert_eq!(p1.is_on_y_axis(), false);
    assert_eq!(p2.is_on_y_axis(), false);
}

#[test]
fn should_calculate_slope() {
    let p1 = PPoint { r: 1.0, theta: 2.0 };
    let p2 = PPoint { r: 3.0, theta: 4.0 };

    assert_eq!(p1.slope(&p2), 2.058349166707094);
    assert_eq!(p2.slope(&p1), 2.058349166707094);

    let m = p1.slope(&p1);
    assert_eq!(m.is_nan(), true);

    let m = p2.slope(&p2);
    assert_eq!(m.is_nan(), true);
}
