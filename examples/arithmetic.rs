use numseries::series::arithmetic::arithmetic_sequence;

fn main() {
    println!("Arithmetic sequence of 10 numbers starting from 0 with a progression of 1:");
    arithmetic_sequence(0f64, 1f64, 10)
        .iter()
        .for_each(|n| print!("{}, ", n));
    println!();

    println!("Arithmetic sequence of 10 numbers starting from 5.4 with a progression of -2.9233:");
    arithmetic_sequence(5.4, -2.9233, 10)
        .iter()
        .for_each(|n| print!("{}, ", n));
}
