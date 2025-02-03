use numberlab::sequence::figurate::square::{nth_square, square_sequence};

fn main() {
    let series = square_sequence(10);
    println!("{:?} == {}", series, series.iter().sum::<u128>());

    let n_series: Vec<u128> = (1..=10).map(|n| nth_square(n)).collect();
    println!("{:?} == {}", n_series, n_series.iter().sum::<u128>());
}
