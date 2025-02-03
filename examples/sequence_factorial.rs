use num::BigUint;
use numberlab::sequence::factorial::{factorial_sequence, factorial_series, nth_factorial};

fn main() {
    let series = factorial_sequence(10);
    println!("{:?} == {}", series, series.iter().sum::<BigUint>());

    let n_series: Vec<BigUint> = (1..=10).map(|n| nth_factorial(n)).collect();
    println!("{:?} == {}", n_series, n_series.iter().sum::<BigUint>());

    println!("{}", factorial_series(10));
}
