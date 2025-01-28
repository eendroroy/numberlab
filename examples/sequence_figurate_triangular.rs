use num_bigint::BigUint;
use numberlab::sequence::figurate::triangular::{nth_triangular, triangular_sequence};

fn main() {
    let series = triangular_sequence(10);
    println!("{:?} == {}", series, series.iter().sum::<BigUint>());

    let n_series: Vec<BigUint> = (0..10).map(|n| nth_triangular(n)).collect();
    println!("{:?} == {}", n_series, n_series.iter().sum::<BigUint>());
}
