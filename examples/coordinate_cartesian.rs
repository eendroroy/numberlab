use numberlab::coordinate::cartesian::{CPoint, CPointTrait};

fn main() {
    let p1 = CPoint { x: 1.75, y: 2.75 };
    let p2 = CPoint { x: 9.05, y: 8.05 };

    println!("Polar:    [{}] {:?}", p1, p1.to_polar());
    println!("Polar:    [{}] {:?}", p2, p2.to_polar());
    println!();
    println!("Distance: [{}]<>[{}] {}", p1, p2, p1.distance(&p2));
    println!("Squared : [{}]<>[{}] {}", p1, p2, p1.distance_squared(&p2));
    println!("Midpoint: [{}]<>[{}] {}", p1, p2, p1.midpoint(&p2));
    println!("Origin:   [{}] {}", p1, p1.is_origin());
    println!("X-axis:   [{}] {}", p1, p1.is_on_x_axis());
    println!("Y-axis:   [{}] {}", p1, p1.is_on_y_axis());
    println!("Slope:    [{}]<>[{}] {}", p1, p2, p1.slope(&p2));
    println!("SlopeInt: [{}]<>[{}] {:?}", p1, p2, p1.slope_intercept(&p2));
}
