/// Calculates the nth Tribonacci number.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the Tribonacci sequence.
///
/// # Returns
///
/// The nth Tribonacci number .
///
/// # Examples
///
/// ```
/// use numberlab::sequence::tribonacci::nth_tribonacci;
///
/// let result = nth_tribonacci(10);
/// assert_eq!(result, 149);
/// ```
pub fn nth_tribonacci(n: usize) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        2 => 1,
        _ => nth_tribonacci(n - 3) + nth_tribonacci(n - 2) + nth_tribonacci(n - 1),
    }
}

/// Calculates the nth Tribonacci number using memoization.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the Tribonacci sequence.
///
/// # Returns
///
/// The nth Tribonacci number .
///
/// # Examples
///
/// ```
/// use numberlab::sequence::tribonacci::nth_tribonacci_memoized;
///
/// let result = nth_tribonacci_memoized(10);
/// assert_eq!(result, 149);
/// ```
pub fn nth_tribonacci_memoized(n: usize) -> u128 {
    let mut memoizer = vec![0, 1, 1];
    nth_tribonacci_with_memoizer(n, &mut memoizer)
}

fn nth_tribonacci_with_memoizer(n: usize, memoizer: &mut Vec<u128>) -> u128 {
    if memoizer.len() > n {
        memoizer[n].clone()
    } else {
        let kth_tribonacci = nth_tribonacci_with_memoizer(n - 3, memoizer);
        let lth_tribonacci = nth_tribonacci_with_memoizer(n - 2, memoizer);
        let mth_tribonacci = nth_tribonacci_with_memoizer(n - 1, memoizer);
        let nth_tribonacci = kth_tribonacci + lth_tribonacci + mth_tribonacci;
        memoizer.push(nth_tribonacci);
        memoizer[n].clone()
    }
}

/// Generates a sequence of Tribonacci numbers up to the given number `n`.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the number of terms in the Tribonacci sequence.
///
/// # Returns
///
/// A vector containing the Tribonacci sequence up to the given number `n`.
///
/// # Examples
///
/// ```
/// use numberlab::sequence::tribonacci::tribonacci_sequence;
///
/// let sequence = tribonacci_sequence(5);
/// assert_eq!(
///     sequence,
///     vec![
///         (0_u128),
///         (1_u128),
///         (1_u128),
///         (2_u128),
///         (4_u128)
///     ]
/// );
/// ```
pub fn tribonacci_sequence(n: usize) -> Vec<u128> {
    match n {
        0 => vec![],
        1 => vec![(0u128)],
        2 => vec![(0u128), (1u128)],
        3 => vec![(0u128), (1u128), (1u128)],
        _ => {
            let mut sequence = vec![(0u128), (1u128), (1u128)];
            (3..n).for_each(|i| {
                sequence.push(&sequence[i - 1] + &sequence[i - 2] + &sequence[i - 3])
            });
            sequence
        }
    }
}
