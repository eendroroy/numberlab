use numseries::series::arithmetic::{arithmetic_sequence, arithmetic_sum};

fn main() {
    println!("Arithmetic sequence of 10 numbers starting from 5.4 with a progression of -2.9233:");
    arithmetic_sequence(1.0, 1.0, 10)
        .iter()
        .for_each(|n| print!("{}, ", n));

    println!();
    println!("Arithmetic sum of 10 numbers starting from 5.4 with a progression of -2.9233:");
    print!("{}", arithmetic_sum(5.4, -2.9233, 10));
}
