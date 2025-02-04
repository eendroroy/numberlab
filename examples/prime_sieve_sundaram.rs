use numberlab::prime::sieve::sundaram_sequence;

fn main() {
    sundaram_sequence(100)
        .iter()
        .for_each(|prime| println!("{}", prime));
}
