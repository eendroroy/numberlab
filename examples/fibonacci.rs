use num_bigint::BigUint;
use numseries::series::fibonacci::{fibonacci_sequence, nth_fibonacci_memoized};

fn main() {
    let mut memoizer = vec![];
    println!("11th Fibonacci: {}", nth_fibonacci_memoized(10, &mut memoizer));

    fibonacci_sequence(BigUint::from(0u128), 11)
        .iter()
        .for_each(|n| println!("{}", n));
}
