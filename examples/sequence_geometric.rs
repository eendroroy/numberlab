use numberlab::sequence::geometric::{geometric_sequence, geometric_series, nth_geometric};

fn main() {
    let series = geometric_sequence(1.12, 2.23, 10);
    println!("{:?} == {}", series, series.iter().sum::<f64>());

    let n_series: Vec<f64> = (1..=10).map(|n| nth_geometric(1.12, 2.23, n)).collect();
    println!("{:?} == {}", n_series, n_series.iter().sum::<f64>());

    println!("{}", geometric_series(1.12, 2.23, 10));
}
