use numberlab::coordinate::cartesian::{Line1D, Line1DTrait, Point1D, Point1DTrait};

fn main() {
    let p1 = Point1D { x: 1.75 };
    let p2 = Point1D { x: 9.05 };

    println!("Distance: [{}]<>[{}] {}", p1, p1, p1.distance(&p1));
    println!("Distance: [{}]<>[{}] {}", p1, p2, p1.distance(&p2));
    println!("Distance: [{}]<>[{}] {}", p2, p1, p2.distance(&p1));
    println!("Distance: [{}]<>[{}] {}", p2, p2, p2.distance(&p2));

    println!();

    println!("Position: [{}] v=1.0 t=2.0 = {}", p1, p1.position(1.0, 2.0));
    println!("Position: [{}] v=1.5 t=2.5 = {}", p2, p2.position(1.5, 2.5));

    let line = Line1D { start: p1, end: p2 };

    println!();

    println!("Line:     {}", line);
    println!("Length:   {}", line.length());
    println!("Midpoint: {}", line.midpoint());
}
