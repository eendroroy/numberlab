use numberlab::coordinate::cartesian::{Line1D, Line1DTrait, Point1D, Point1DTrait};

#[test]
fn should_display_point() {
    let p = Point1D { x: 1.0 };

    assert_eq!(format!("{}", p), "{ x: 1 }");
}

#[test]
fn should_calculate_distance() {
    let p1 = Point1D { x: 1.0 };
    let p2 = Point1D { x: 3.0 };

    assert_eq!(p1.distance(&p1), 0.0);
    assert_eq!(p1.distance(&p2), 2.0);
    assert_eq!(p2.distance(&p1), 2.0);
    assert_eq!(p2.distance(&p2), 0.0);
}

#[test]
fn should_calculate_position() {
    let p1 = Point1D { x: 1.0 };
    let p2 = Point1D { x: 3.0 };

    assert_eq!(p1.position(1.0, 2.0).x, 3.0);
    assert_eq!(p2.position(1.5, 2.5).x, 6.75);
}

#[test]
fn should_display_line() {
    let p1 = Point1D { x: 1.0 };
    let p2 = Point1D { x: 3.0 };
    let line = Line1D { start: p1, end: p2 };

    assert_eq!(format!("{}", line), "{ start: { x: 1 }, end: { x: 3 } }");
}

#[test]
fn should_calculate_length() {
    let p1 = Point1D { x: 1.0 };
    let p2 = Point1D { x: 3.0 };
    let line = Line1D { start: p1, end: p2 };

    assert_eq!(line.length(), 2.0);
}

#[test]
fn should_calculate_midpoint() {
    let p1 = Point1D { x: 1.0 };
    let p2 = Point1D { x: 3.0 };
    let line = Line1D { start: p1, end: p2 };

    assert_eq!(line.midpoint().x, 2.0);
}
