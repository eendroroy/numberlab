use numseries::series::geometric::{geometric_sequence, geometric_sum};

fn main() {
    println!("Geometric sequence of 10 numbers starting from 1.5 with a progression of -1.5:");
    geometric_sequence(1.5, -1.5, 10)
        .iter()
        .for_each(|n| print!("{}, ", n));

    println!("Geometric sum of 10 numbers starting from 1.5 with a progression of -1.5:");
    print!("{}", geometric_sum(1.5, -1.5, 10));
}
