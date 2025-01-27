use numberlab::prime::sieve::sieve_prime_sequence;

fn main() {
    let n = 10000;
    sieve_prime_sequence(n).iter().for_each(
        |prime| println!("{}", prime)
    );
}