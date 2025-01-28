use num_bigint::BigUint;
use numberlab::sequence::sylvester::{nth_sylvester, sylvester_sequence};

fn main() {
    let series = sylvester_sequence(7);
    println!("{:?} == {}", series, series.iter().sum::<BigUint>());

    let n_series: Vec<BigUint> = (0..7).map(|n| nth_sylvester(n)).collect();
    println!("{:?} == {}", n_series, n_series.iter().sum::<BigUint>());
}
