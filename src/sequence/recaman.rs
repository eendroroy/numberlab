/// Calculates the nth Recaman number using memoization.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the Recaman sequence.
///
/// # Returns
///
/// The nth Recaman number as a `u128`.
///
/// # Examples
///
/// ```
/// use numberlab::sequence::recaman::nth_recaman_memoized;
///
/// let result = nth_recaman_memoized(10);
/// assert_eq!(result, 11);
/// ```
pub fn nth_recaman_memoized(n: usize) -> u128 {
    let mut memoizer = vec![0];
    nth_recaman_with_memoizer(n, &mut memoizer)
}

fn nth_recaman_with_memoizer(n: usize, memoizer: &mut Vec<u128>) -> u128 {
    if memoizer.len() <= n {
        let mth_recaman = nth_recaman_with_memoizer(n - 1, memoizer);
        let nth_recaman =
            if mth_recaman > n as u128 && !memoizer.contains(&(mth_recaman - n as u128)) {
                mth_recaman - n as u128
            } else {
                mth_recaman + n as u128
            };
        memoizer.push(nth_recaman);
    }
    memoizer[n]
}

/// Generates a sequence of Recaman numbers up to the given number `n`.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the number of terms in the Recaman sequence.
///
/// # Returns
///
/// A vector containing the Recaman sequence up to the given number `n`.
///
/// # Examples
///
/// ```
/// use numberlab::sequence::recaman::recaman_sequence;
///
/// let sequence = recaman_sequence(10);
/// assert_eq!(sequence, vec![0, 1, 3, 6, 2, 7, 13, 20, 12, 21]);
/// ```
pub fn recaman_sequence(n: usize) -> Vec<u128> {
    let mut sequence = vec![0];
    nth_recaman_with_memoizer(n - 1, &mut sequence);
    sequence
}
