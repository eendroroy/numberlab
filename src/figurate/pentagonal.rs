use std::ops::{Div, Mul, Sub};

/// Calculates the nth pentagonal number.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the pentagonal sequence.
///
/// # Returns
///
/// The nth pentagonal number .
///
/// # Examples
///
/// ```
/// use numberlab::figurate::pentagonal::nth_pentagonal;
///
/// let result = nth_pentagonal(10);
/// assert_eq!(result, 145);
/// ```
pub fn nth_pentagonal(n: u128) -> u128 {
    match n {
        0 => 0,
        _ => n.mul(n.mul(3).sub(1)).div(2),
    }
}

/// Generates a sequence of pentagonal numbers up to the given number `n`.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the number of terms in the pentagonal sequence.
///
/// # Returns
///
/// A vector containing the pentagonal sequence up to the given number `n`.
///
/// # Examples
///
/// ```
/// use numberlab::figurate::pentagonal::pentagonal_sequence;
///
/// let sequence = pentagonal_sequence(6);
/// assert_eq!(sequence, vec![0, 1, 5, 12, 22, 35]);
/// ```
pub fn pentagonal_sequence(n: usize) -> Vec<u128> {
    (0..n as u128).map(nth_pentagonal).collect()
}
