/// Generates a sequence of prime numbers up to a given number `n` using the Sieve of Sundaram algorithm.
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
/// use numberlab::prime::sieve::sundaram::sundaram_sequence;
///
/// let primes = sundaram_sequence(30);
/// assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
/// ```
pub fn sundaram_sequence(n: usize) -> Vec<usize> {
    let k = (n - 2) / 2;
    let mut sieve = vec![true; k + 1];
    for i in 1..k + 1 {
        let mut j = i;
        while i + j + 2 * i * j <= k {
            sieve[i + j + 2 * i * j] = false;
            j += 1;
        }
    }
    let mut primes = vec![2];
    for i in 1..k + 1 {
        if sieve[i] {
            primes.push(2 * i + 1);
        }
    }
    primes
}
