use crate::prime::sieve::eratosthenes::eratosthenes_sequence;
use std::collections::BTreeMap;

/// Returns a vector of factor pairs of the given number `n`.
///
/// # Arguments
///
/// * `n` - A usize integer whose factor pairs are to be found.
///
/// # Returns
///
/// A vector of tuples where each tuple contains a pair of factors of `n`.
///
/// # Examples
///
/// ```
/// use numberlab::algorithm::factorize::factor_pairs;
///
/// let pairs = factor_pairs(28);
/// assert_eq!(pairs, vec![(1, 28), (2, 14), (4, 7)]);
/// ```
pub fn factor_pairs(n: usize) -> Vec<(usize, usize)> {
    if n == 0 {
        return vec![];
    }
    let mut pairs = vec![(1, n)];
    for start in 2.. {
        if start * start > n {
            break;
        }
        if n % start == 0 {
            pairs.push((start, n / start));
        }
    }
    pairs
}

/// Returns a sorted vector of all factors of the given number `n`.
///
/// # Arguments
///
/// * `n` - A usize integer whose factors are to be found.
///
/// # Returns
///
/// A sorted vector of usize integers representing all factors of `n`.
///
/// # Examples
///
/// ```
/// use numberlab::algorithm::factorize::factors;
///
/// let factors = factors(28);
/// assert_eq!(factors, vec![1, 2, 4, 7, 14, 28]);
/// ```
pub fn factors(n: usize) -> Vec<usize> {
    let mut factors = factor_pairs(n)
        .into_iter()
        .flat_map(|(a, b)| if a == b { vec![a] } else { vec![a, b] })
        .collect::<Vec<usize>>();
    factors.sort_unstable();
    factors
}

/// Returns a vector of prime factors of the given number `n`.
///
/// # Arguments
///
/// * `n` - A usize integer whose prime factors are to be found.
///
/// # Returns
///
/// A vector of usize integers representing the prime factors of `n`.
///
/// # Examples
///
/// ```
/// use numberlab::algorithm::factorize::prime_factors;
///
/// let factors = prime_factors(28);
/// assert_eq!(factors, vec![2, 2, 7]);
/// ```
pub fn prime_factors(n: usize) -> Vec<usize> {
    if n == 0 {
        return vec![];
    }
    let mut primes = eratosthenes_sequence(n).into_iter();
    let mut factors = vec![];
    let mut current = n;
    while current > 1 {
        let prime = primes.next().unwrap();
        while current % prime == 0 {
            factors.push(prime);
            current /= prime;
        }
    }
    factors
}

/// Returns a `BTreeMap` where the keys are the prime factors of the given number `n`
/// and the values are the exponents of those prime factors.
///
/// # Arguments
///
/// * `n` - A usize integer whose prime factors and their exponents are to be found.
///
/// # Returns
///
/// A `BTreeMap<usize, usize>` where each key is a prime factor of `n` and the corresponding
/// value is the exponent of that prime factor.
///
/// # Examples
///
/// ```
/// use numberlab::algorithm::factorize::prime_factors_exponent;
///
/// let exponents = prime_factors_exponent(28);
/// assert_eq!(exponents.get(&2), Some(&2));
/// assert_eq!(exponents.get(&7), Some(&1));
/// ```
pub fn prime_factors_exponent(n: usize) -> BTreeMap<usize, usize> {
    let mut map = BTreeMap::new();
    for &factor in &prime_factors(n) {
        *map.entry(factor).or_insert(0) += 1;
    }
    map
}
