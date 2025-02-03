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
/// use numberlab::sequence::fibonacci::nth_fibonacci;
///
/// let result = nth_fibonacci(10);
/// assert_eq!(result, 55);
/// ```
pub fn nth_fibonacci(n: usize) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => nth_fibonacci(n - 1) + nth_fibonacci(n - 2),
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
/// use numberlab::sequence::fibonacci::nth_fibonacci_memoized;
///
/// let result = nth_fibonacci_memoized(10);
/// assert_eq!(result, 55);
/// ```
pub fn nth_fibonacci_memoized(n: usize) -> u128 {
    let mut memoizer = vec![0u128, 1u128];
    nth_fibonacci_with_memoizer(n, &mut memoizer)
}

fn nth_fibonacci_with_memoizer(n: usize, memoizer: &mut Vec<u128>) -> u128 {
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
/// use numberlab::sequence::fibonacci::fibonacci_sequence;
///
/// let sequence = fibonacci_sequence(5);
/// assert_eq!(sequence, vec![ 0, 1, 1, 2, 3]);
/// ```
pub fn fibonacci_sequence(n: usize) -> Vec<u128> {
    if n == 0 {
        vec![]
    } else if n == 1 {
        vec![0]
    } else if n == 2 {
        vec![0, 1]
    } else {
        let mut sequence = vec![0, 1];
        (2..n).for_each(|i| sequence.push(&sequence[i - 1] + &sequence[i - 2]));
        sequence
    }
}
