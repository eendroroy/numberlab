use numberlab::sequence::figurate::hexagonal::{hexagonal_sequence, nth_hexagonal};

fn main() {
    let sequence = hexagonal_sequence(10);
    println!("{:?} == {}", sequence, sequence.iter().sum::<u128>());

    let n_sequence: Vec<u128> = (1..=10).map(|n| nth_hexagonal(n)).collect();
    println!("{:?} == {}", n_sequence, n_sequence.iter().sum::<u128>());
}
