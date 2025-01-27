use num_bigint::BigUint;
use std::ops::Mul;

/// Calculates the factorial of a given number `n`.
///
/// # Arguments
///
/// * `n` - A positive integer representing the number to calculate the factorial of.
///
/// # Panics
///
/// This function will panic if `n` is less than 1.
///
/// # Examples
///
/// ```
/// use num_bigint::BigUint;
/// use numberlab::sequence::factorial::nth_factorial;
///
/// let result = nth_factorial(5);
/// assert_eq!(result, BigUint::from(120_u128));
/// ```
pub fn nth_factorial(n: usize) -> BigUint {
    if n < 1 {
        panic!("'n' must be greater than 0");
    }
    (1..=n)
        .map(|i| BigUint::from(i))
        .fold(BigUint::from(1u128), |acc, x| acc.mul(x))
}

/// Calculates the factorial of a given number `n` using memoization.
///
/// # Arguments
///
/// * `n` - A positive integer representing the number to calculate the factorial of.
///
/// # Panics
///
/// This function will panic if `n` is less than 1.
///
/// # Examples
///
/// ```
/// use num_bigint::BigUint;
/// use numberlab::sequence::factorial::nth_factorial_memoized;
///
/// let result = nth_factorial_memoized(5);
/// assert_eq!(result, BigUint::from(120_u128));
/// ```
pub fn nth_factorial_memoized(n: usize) -> BigUint {
    if n < 1 {
        panic!("'n' must be greater than 0");
    }
    let mut memoizer = vec![BigUint::from(1u128)];
    nth_factorial_with_memoizer(n, &mut memoizer)
}

fn nth_factorial_with_memoizer(n: usize, memoizer: &mut Vec<BigUint>) -> BigUint {
    if n < memoizer.len() {
        memoizer[n].clone()
    } else {
        let mth_factorial = nth_factorial_with_memoizer(n - 1, memoizer);
        let nth_factorial = mth_factorial.mul(n);
        memoizer.push(nth_factorial);
        memoizer[n].clone()
    }
}

/// Generates a sequence of factorials up to the given number `n`.
///
/// # Arguments
///
/// * `n` - A positive integer representing the number of terms in the factorial sequence.
///
/// # Panics
///
/// This function will panic if `n` is less than 1.
///
/// # Examples
///
/// ```
/// use num_bigint::BigUint;
/// use numberlab::sequence::factorial::factorial_sequence;
///
/// let sequence = factorial_sequence(5);
/// assert_eq!(
///     sequence,
///     vec![
///         BigUint::from(1_u128),
///         BigUint::from(2_u128),
///         BigUint::from(6_u128),
///         BigUint::from(24_u128),
///         BigUint::from(120_u128)
///     ]
/// );
/// ```
pub fn factorial_sequence(n: usize) -> Vec<BigUint> {
    if n < 1 {
        panic!("'n' must be greater than 0");
    }
    if n == 1 {
        vec![BigUint::from(1u128)]
    } else {
        let mut sequence = vec![BigUint::from(1u128)];
        (1..n).for_each(|i| sequence.push(&sequence[i - 1] * BigUint::from(i + 1)));
        sequence
    }
}

/// Calculates the sum of the factorials up to the given number `n`.
///
/// # Arguments
///
/// * `n` - A positive integer representing the number of terms in the factorial sequence.
///
/// # Returns
///
/// The sum of the factorials up to the given number `n`.
///
/// # Examples
///
/// ```
/// use num_bigint::BigUint;
/// use numberlab::sequence::factorial::factorial_series;
///
/// let result = factorial_series(5);
/// assert_eq!(result, BigUint::from(153_u128)); // 1! + 2! + 3! + 4! + 5! = 153
/// ```
pub fn factorial_series(n: usize) -> BigUint {
    if n < 1 {
        return BigUint::from(0u128);
    }
    factorial_sequence(n).iter().sum()
}

#[cfg(test)]
mod factorial_test;
