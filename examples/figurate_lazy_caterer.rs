use numberlab::figurate::lazy_caterer::{lazy_caterer_sequence, nth_lazy_caterer};

fn main() {
    let sequence = lazy_caterer_sequence(10);
    println!("{:?} == {}", sequence, sequence.iter().sum::<u128>());

    let n_sequence: Vec<u128> = (0..10).map(|n| nth_lazy_caterer(n)).collect();
    println!("{:?} == {}", n_sequence, n_sequence.iter().sum::<u128>());
}
