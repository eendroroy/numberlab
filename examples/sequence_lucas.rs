use num::BigUint;
use numberlab::sequence::lucas::{lucas_sequence, nth_lucas};

fn main() {
    let series = lucas_sequence(10);
    println!("{:?} == {}", series, series.iter().sum::<BigUint>());

    let n_series: Vec<BigUint> = (0..10).map(|n| nth_lucas(n)).collect();
    println!("{:?} == {}", n_series, n_series.iter().sum::<BigUint>());
}
