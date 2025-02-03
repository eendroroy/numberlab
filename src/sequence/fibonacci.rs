use num::BigUint;

/// Calculates the nth Fibonacci number.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the Fibonacci sequence.
///
/// # Returns
///
/// The nth Fibonacci number .
///
/// # Examples
///
/// ```
/// use num::BigUint;
/// use numberlab::sequence::fibonacci::nth_fibonacci;
///
/// let result = nth_fibonacci(10);
/// assert_eq!(result, BigUint::from(55u128));
/// ```
pub fn nth_fibonacci(n: usize) -> BigUint {
    if n == 0 {
        BigUint::from(0u128)
    } else if n == 1 {
        BigUint::from(1u128)
    } else {
        nth_fibonacci(n - 1) + nth_fibonacci(n - 2)
    }
}

/// Calculates the nth Fibonacci number using memoization.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the Fibonacci sequence.
///
/// # Returns
///
/// The nth Fibonacci number .
///
/// # Examples
///
/// ```
/// use num::BigUint;
/// use numberlab::sequence::fibonacci::nth_fibonacci_memoized;
///
/// let result = nth_fibonacci_memoized(10);
/// assert_eq!(result, BigUint::from(55u128));
/// ```
pub fn nth_fibonacci_memoized(n: usize) -> BigUint {
    let mut memoizer = vec![BigUint::from(0u128), BigUint::from(1u128)];
    nth_fibonacci_with_memoizer(n, &mut memoizer)
}

fn nth_fibonacci_with_memoizer(n: usize, memoizer: &mut Vec<BigUint>) -> BigUint {
    if memoizer.len() > n {
        memoizer[n].clone()
    } else {
        let lth_fibonacci = nth_fibonacci_with_memoizer(n - 1, memoizer);
        let mth_fibonacci = nth_fibonacci_with_memoizer(n - 2, memoizer);
        let nth_fibonacci = lth_fibonacci + mth_fibonacci;
        memoizer.push(nth_fibonacci);
        memoizer[n].clone()
    }
}

/// Generates a sequence of Fibonacci numbers up to the given number `n`.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the number of terms in the Fibonacci sequence.
///
/// # Returns
///
/// A vector containing the Fibonacci sequence up to the given number `n`.
///
/// # Examples
///
/// ```
/// use num::BigUint;
/// use numberlab::sequence::fibonacci::fibonacci_sequence;
///
/// let sequence = fibonacci_sequence(5);
/// assert_eq!(
///     sequence,
///     vec![
///         BigUint::from(0_u128),
///         BigUint::from(1_u128),
///         BigUint::from(1_u128),
///         BigUint::from(2_u128),
///         BigUint::from(3_u128)
///     ]
/// );
/// ```
pub fn fibonacci_sequence(n: usize) -> Vec<BigUint> {
    if n == 0 {
        vec![]
    } else if n == 1 {
        vec![BigUint::from(0u128)]
    } else if n == 2 {
        vec![BigUint::from(0u128), BigUint::from(1u128)]
    } else {
        let mut sequence = vec![BigUint::from(0u128), BigUint::from(1u128)];
        (2..n).for_each(|i| sequence.push(&sequence[i - 1] + &sequence[i - 2]));
        sequence
    }
}

/// Calculates the sum of the first `n` Fibonacci numbers.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the number of terms in the Fibonacci sequence.
///
/// # Returns
///
/// The sum of the first `n` Fibonacci numbers .
///
/// # Examples
///
/// ```
/// use num::BigUint;
/// use numberlab::sequence::fibonacci::fibonacci_series;
///
/// let result = fibonacci_series(10);
/// assert_eq!(result, BigUint::from(88u128));
/// ```
pub fn fibonacci_series(n: usize) -> BigUint {
    fibonacci_sequence(n).iter().sum()
}
