/// Calculates the nth triangular number.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the triangular sequence.
///
/// # Returns
///
/// The nth triangular number.
///
/// # Examples
///
/// ```
/// use numberlab::figurate::triangular::nth_triangular;
///
/// let result = nth_triangular(10);
/// assert_eq!(result, 55);
/// ```
pub fn nth_triangular(n: u128) -> u128 {
    n * (n + 1) / 2
}

/// Generates a sequence of triangular numbers up to the given number `n`.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the number of terms in the triangular sequence.
///
/// # Returns
///
/// A vector containing the triangular sequence up to the given number `n`.
///
/// # Examples
///
/// ```
/// use numberlab::figurate::triangular::triangular_sequence;
///
/// let sequence = triangular_sequence(6);
/// assert_eq!(sequence, vec![0, 1, 3, 6, 10, 15]);
/// ```
pub fn triangular_sequence(n: usize) -> Vec<u128> {
    (0..n as u128).map(nth_triangular).collect()
}
