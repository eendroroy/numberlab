use numberlab::figurate::star::{nth_star, star_sequence};

fn main() {
    let sequence = star_sequence(10);
    println!("{:?} == {}", sequence, sequence.iter().sum::<u128>());

    let n_sequence: Vec<u128> = (1..=10).map(|n| nth_star(n)).collect();
    println!("{:?} == {}", n_sequence, n_sequence.iter().sum::<u128>());
}
