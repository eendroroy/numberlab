use num_bigint::BigUint;
use numseries::series::fibonacci::{fibonacci_sequence, fibonacci_series, nth_fibonacci};

fn main() {
    let series = fibonacci_sequence(10);
    println!("{:?} == {}", series, series.iter().sum::<BigUint>());

    let n_series: Vec<BigUint> = (0..10).map(|n| nth_fibonacci(n)).collect();
    println!("{:?} == {}", n_series, n_series.iter().sum::<BigUint>());

    println!("{}", fibonacci_series(10));
}
