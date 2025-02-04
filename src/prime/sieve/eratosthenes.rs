/// Generates a sequence of prime numbers up to a given number `n` using the Sieve of Eratosthenes algorithm.
///
/// # Arguments
///
/// * `n` - The upper limit of the range to generate prime numbers (inclusive).
///
/// # Returns
///
/// A vector containing all prime numbers up to `n`.
///
/// # Examples
///
/// ```
/// use numberlab::prime::sieve::eratosthenes::eratosthenes_sequence;
///
/// let primes = eratosthenes_sequence(30);
/// assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
/// ```
pub fn eratosthenes_sequence(n: usize) -> Vec<usize> {
    let mut primes = vec![];
    let mut sieve = vec![true; n + 1];
    for i in 2..n + 1 {
        if sieve[i] {
            primes.push(i);
            let mut j = i * i;
            while j < n + 1 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    primes
}
