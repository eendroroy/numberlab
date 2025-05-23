use numberlab::prime::sieve::eratosthenes_sequence;

fn main() {
    eratosthenes_sequence(10000)
        .iter()
        .for_each(|prime| println!("{}", prime));
}
