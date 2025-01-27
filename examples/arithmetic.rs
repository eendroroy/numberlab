use numseries::series::arithmetic::{arithmetic_sequence, arithmetic_sum, nth_arithmetic};

fn main() {
    let series = arithmetic_sequence(1.0, 1.0, 10);
    println!(
        "{:?} == {}",
        series,
        series.iter().fold(0.0, |acc, n| acc + n)
    );

    let n_series = (1..=10)
        .map(|n| nth_arithmetic(1.0, 1.0, n))
        .collect::<Vec<f64>>();
    println!(
        "{:?} == {}",
        n_series,
        n_series.iter().fold(0.0, |acc, n| acc + n)
    );

    println!("{}", arithmetic_sum(1.0, 1.0, 10));
}
