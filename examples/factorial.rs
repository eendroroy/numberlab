use numseries::series::factorial::{factorial_sequence, nth_factorial_with_memoizer};
use numseries::series::fibonacci::nth_fibonacci_memoized;

fn main() {
    println!("11! : {}", nth_factorial_with_memoizer(10));

    factorial_sequence(10)
        .iter()
        .for_each(|n| println!("{}", n));
}
