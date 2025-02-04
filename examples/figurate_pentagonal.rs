use numberlab::figurate::pentagonal::{nth_pentagonal, pentagonal_sequence};

fn main() {
    let sequence = pentagonal_sequence(10);
    println!("{:?} == {}", sequence, sequence.iter().sum::<u128>());

    let n_sequence: Vec<u128> = (1..=10).map(|n| nth_pentagonal(n)).collect();
    println!("{:?} == {}", n_sequence, n_sequence.iter().sum::<u128>());
}
