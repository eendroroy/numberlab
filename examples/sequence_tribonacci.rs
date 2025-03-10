use numberlab::sequence::tribonacci::{nth_tribonacci_memoized, tribonacci_sequence};

fn main() {
    let series = tribonacci_sequence(10);
    println!("{:?} == {}", series, series.iter().sum::<u128>());

    let n_series: Vec<u128> = (0..10).map(|n| nth_tribonacci_memoized(n)).collect();
    println!("{:?} == {}", n_series, n_series.iter().sum::<u128>());
}
