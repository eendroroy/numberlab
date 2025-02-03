use numberlab::sequence::sylvester::{nth_sylvester, sylvester_sequence};

fn main() {
    let series = sylvester_sequence(7);
    println!("{:?} == {}", series, series.iter().sum::<u128>());

    let n_series: Vec<u128> = (0..7).map(|n| nth_sylvester(n)).collect();
    println!("{:?} == {}", n_series, n_series.iter().sum::<u128>());
}
