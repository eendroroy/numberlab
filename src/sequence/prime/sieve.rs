pub fn sieve_prime_sequence(n: u128) -> Vec<u128> {
    let mut primes = vec![];
    let mut sieve = vec![true; n as usize + 1];
    for i in 2..n as usize + 1 {
        if sieve[i] {
            primes.push(i as u128);
            let mut j = i * i;
            while j < n as usize + 1 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    primes
}
