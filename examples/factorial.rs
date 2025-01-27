use num_bigint::BigUint;
use numseries::series::factorial::{factorial_sequence, nth_factorial};

fn main() {
    let series = factorial_sequence(10);
    println!(
        "{:?} == {}",
        series,
        series.iter().fold(BigUint::from(0_u128), |acc, x| acc + x)
    );

    let n_series: Vec<BigUint> = (1..=10).map(|n| nth_factorial(n)).collect();
    println!(
        "{:?} == {}",
        n_series,
        n_series.iter().fold(BigUint::from(0_u128), |acc, x| acc + x)
    );
}
