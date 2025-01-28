use num_bigint::BigUint;
use numberlab::sequence::tribonacci::{nth_tribonacci_memoized, tribonacci_sequence};

fn main() {
    let series = tribonacci_sequence(10);
    println!("{:?} == {}", series, series.iter().sum::<BigUint>());

    let n_series: Vec<BigUint> = (0..10).map(|n| nth_tribonacci_memoized(n)).collect();
    println!("{:?} == {}", n_series, n_series.iter().sum::<BigUint>());
}
