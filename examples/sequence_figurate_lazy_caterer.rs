use num::BigUint;
use numberlab::sequence::figurate::lazy_caterer::{lazy_caterer_sequence, nth_lazy_caterer};

fn main() {
    let sequence = lazy_caterer_sequence(10);
    println!("{:?} == {}", sequence, sequence.iter().sum::<BigUint>());

    let n_sequence: Vec<BigUint> = (0..10).map(|n| nth_lazy_caterer(n)).collect();
    println!("{:?} == {}", n_sequence, n_sequence.iter().sum::<BigUint>());
}
