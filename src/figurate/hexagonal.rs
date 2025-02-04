use std::ops::{Div, Mul, Sub};

/// Calculates the nth hexagonal number.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the hexagonal sequence.
///
/// # Returns
///
/// The nth hexagonal number .
///
/// # Examples
///
/// ```
/// use numberlab::figurate::hexagonal::nth_hexagonal;
///
/// let result = nth_hexagonal(10);
/// assert_eq!(result, 190);
/// ```
pub fn nth_hexagonal(n: u128) -> u128 {
    match n {
        0 => 0,
        _ => n.mul(2).mul(n.mul(2).sub(1)).div(2),
    }
}

/// Generates a sequence of hexagonal numbers up to the given number `n`.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the number of terms in the hexagonal sequence.
///
/// # Returns
///
/// A vector containing the hexagonal sequence up to the given number `n`.
///
/// # Examples
///
/// ```
/// use numberlab::figurate::hexagonal::hexagonal_sequence;
///
/// let sequence = hexagonal_sequence(5);
/// assert_eq!(sequence, vec![0, 1, 6, 15, 28]);
/// ```
pub fn hexagonal_sequence(n: usize) -> Vec<u128> {
    (0..n as u128).map(nth_hexagonal).collect()
}
