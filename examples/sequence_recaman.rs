use num::BigUint;
use numberlab::sequence::recaman::{nth_recaman_memoized, recaman_sequence};

fn main() {
    let series = recaman_sequence(10);
    println!("{:?} == {}", series, series.iter().sum::<BigUint>());

    let n_series: Vec<u128> = (0..10).map(|n| nth_recaman_memoized(n)).collect();
    println!("{:?} == {}", n_series, n_series.iter().sum::<BigUint>());
}
