use num_bigint::BigUint;
use numseries::series::fibonacci::fibonacci_sequence;

fn main() {
    fibonacci_sequence(BigUint::from(0u128), 10)
        .iter()
        .for_each(|n| println!("{}", n));
}
