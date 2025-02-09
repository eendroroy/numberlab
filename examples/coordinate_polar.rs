use numberlab::coordinate::{PPoint, PPointTrait};

fn main() {
    let p1 = PPoint { r: 1.75, theta: 2.75 };
    let p2 = PPoint { r: 9.05, theta: 8.05 };

    println!("Cartesian: [{}] {:?}", p1, p1.to_cartesian());
    println!("Cartesian: [{}] {:?}", p2, p2.to_cartesian());
    println!();
    println!("Distance:  [{}]<>[{}] {}", p1, p2, p1.distance(&p2));
    println!("Squared :  [{}]<>[{}] {}", p1, p2, p1.distance_squared(&p2));
    println!("Midpoint:  [{}]<>[{}] {}", p1, p2, p1.midpoint(&p2));
    println!("Origin:    [{}] {}", p1, p1.is_origin());
    println!("X-axis:    [{}] {}", p1, p1.is_on_x_axis());
    println!("Y-axis:    [{}] {}", p1, p1.is_on_y_axis());
    println!("Slope:     [{}]<>[{}] {}", p1, p2, p1.slope(&p2));
}
