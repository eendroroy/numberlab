use std::ops::{Add, Div};

/// Calculates the nth lazy caterer number.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the lazy caterer sequence.
///
/// # Returns
///
/// The nth lazy caterer number .
///
/// # Examples
///
/// ```
/// use numberlab::figurate::lazy_caterer::nth_lazy_caterer;
///
/// let result = nth_lazy_caterer(10);
/// assert_eq!(result, 56);
/// ```
pub fn nth_lazy_caterer(n: u128) -> u128 {
    n.pow(2).add(n).add(2).div(2)
}

/// Generates a sequence of lazy caterer numbers up to the given number `n`.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the number of terms in the lazy caterer sequence.
///
/// # Returns
///
/// A vector containing the lazy caterer sequence up to the given number `n`.
///
/// # Examples
///
/// ```
/// use numberlab::figurate::lazy_caterer::lazy_caterer_sequence;
///
/// let sequence = lazy_caterer_sequence(5);
/// assert_eq!(sequence, vec![1, 2, 4, 7, 11]);
/// ```
pub fn lazy_caterer_sequence(n: usize) -> Vec<u128> {
    (0..n as u128).map(nth_lazy_caterer).collect()
}
