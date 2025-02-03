use numberlab::sequence::fibonacci::{fibonacci_sequence, nth_fibonacci};

fn main() {
    let series = fibonacci_sequence(10);
    println!("{:?} == {}", series, series.iter().sum::<u128>());

    let n_series: Vec<u128> = (0..10).map(|n| nth_fibonacci(n)).collect();
    println!("{:?} == {}", n_series, n_series.iter().sum::<u128>());
}
