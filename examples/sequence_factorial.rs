use numberlab::formula::arithmetic::factorial;
use numberlab::sequence::factorial::factorial_sequence;

fn main() {
    let series = factorial_sequence(10);
    println!("{:?} == {}", series, series.iter().sum::<u128>());

    let n_series: Vec<u128> = (0..10).map(|n| factorial(n)).collect();
    println!("{:?} == {}", n_series, n_series.iter().sum::<u128>());
}
