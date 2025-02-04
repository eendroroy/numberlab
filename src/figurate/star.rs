/// Calculates the nth star number.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the star sequence.
///
/// # Returns
///
/// The nth star number.
///
/// # Examples
///
/// ```
/// use numberlab::figurate::star::nth_star;
///
/// let result = nth_star(10);
/// assert_eq!(result, 541);
/// ```
pub fn nth_star(n: u128) -> u128 {
    6 * n * (n - 1) + 1
}

/// Generates a sequence of star numbers up to the given number `n`.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the number of terms in the star sequence.
///
/// # Returns
///
/// A vector containing the star sequence up to the given number `n`.
///
/// # Examples
///
/// ```
/// use numberlab::figurate::star::star_sequence;
///
/// let sequence = star_sequence(5);
/// assert_eq!(sequence, vec![1, 13, 37, 73, 121]);
/// ```
pub fn star_sequence(n: usize) -> Vec<u128> {
    (1..=n as u128).map(nth_star).collect()
}
