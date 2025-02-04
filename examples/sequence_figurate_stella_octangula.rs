use numberlab::sequence::figurate::stella_octangula::{nth_stella_octangula, stella_octangula_sequence};

fn main() {
    let sequence = stella_octangula_sequence(10);
    println!("{:?} == {}", sequence, sequence.iter().sum::<u128>());

    let n_sequence: Vec<u128> = (0..10).map(|n| nth_stella_octangula(n)).collect();
    println!("{:?} == {}", n_sequence, n_sequence.iter().sum::<u128>());
}
