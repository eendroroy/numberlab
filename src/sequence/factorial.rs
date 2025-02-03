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
/// use numberlab::sequence::factorial::nth_factorial;
///
/// let result = nth_factorial(5);
/// assert_eq!(result, 120);
/// ```
pub fn nth_factorial(n: u128) -> u128 {
    match n {
        0 => 1,
        _ => (1..=n).product(),
    }
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
/// use numberlab::sequence::factorial::nth_factorial_memoized;
///
/// let result = nth_factorial_memoized(5);
/// assert_eq!(result, u128::from(120_u128));
/// ```
pub fn nth_factorial_memoized(n: usize) -> u128 {
    let mut memoizer = vec![1u128];
    nth_factorial_with_memoizer(n, &mut memoizer)
}

fn nth_factorial_with_memoizer(n: usize, memoizer: &mut Vec<u128>) -> u128 {
    if n < memoizer.len() {
        memoizer[n].clone()
    } else {
        let mth_factorial = nth_factorial_with_memoizer(n - 1, memoizer);
        let nth_factorial = mth_factorial.mul(n as u128);
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
/// use numberlab::sequence::factorial::factorial_sequence;
///
/// let sequence = factorial_sequence(6);
/// assert_eq!(sequence, vec![1, 1, 2, 6, 24, 120]);
/// ```
pub fn factorial_sequence(n: usize) -> Vec<u128> {
    match n {
        0 => vec![],
        1 => vec![1],
        _ => {
            let mut sequence = vec![1u128];
            (1..n).for_each(|i| sequence.push(sequence[i - 1] * i as u128));
            sequence
        }
    }
}
