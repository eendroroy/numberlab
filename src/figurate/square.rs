/// Calculates the nth square number.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the square sequence.
///
/// # Returns
///
/// The nth square number .
///
/// # Examples
///
/// ```
/// use numberlab::figurate::square::nth_square;
///
/// let result = nth_square(10);
/// assert_eq!(result, 100);
/// ```
pub fn nth_square(n: u128) -> u128 {
    n.pow(2)
}

/// Generates a sequence of square numbers up to the given number `n`.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the number of terms in the square sequence.
///
/// # Returns
///
/// A vector containing the square sequence up to the given number `n`.
///
/// # Examples
///
/// ```
/// use numberlab::figurate::square::square_sequence;
///
/// let sequence = square_sequence(6);
/// assert_eq!(sequence, vec![0, 1, 4, 9, 16, 25]);
/// ```
pub fn square_sequence(n: usize) -> Vec<u128> {
    (0..n as u128).map(nth_square).collect()
}
