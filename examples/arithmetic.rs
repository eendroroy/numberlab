use numberlab::series::arithmetic::{arithmetic_sequence, arithmetic_series, nth_arithmetic};

fn main() {
    let series = arithmetic_sequence(1.0, 1.0, 10);
    println!("{:?} == {}", series, series.iter().sum::<f64>());

    let n_series: Vec<f64> = (1..=10).map(|n| nth_arithmetic(1.0, 1.0, n)).collect();
    println!("{:?} == {}", n_series, n_series.iter().sum::<f64>());

    println!("{}", arithmetic_series(1.0, 1.0, 10));
}
