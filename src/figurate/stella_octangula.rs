/// Calculates the nth stella octangula number.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the stella octangula sequence.
///
/// # Returns
///
/// The nth stella octangula number.
///
/// # Examples
///
/// ```
/// use numberlab::figurate::stella_octangula::nth_stella_octangula;
///
/// let result = nth_stella_octangula(10);
/// assert_eq!(result, 1990);
/// ```
pub fn nth_stella_octangula(n: u128) -> u128 {
    match n {
        0 => 0,
        _ => n * (2 * n * n - 1),
    }
}

/// Generates a sequence of stella octangula numbers up to the given number `n`.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the number of terms in the stella octangula sequence.
///
/// # Returns
///
/// A vector containing the stella octangula sequence up to the given number `n`.
///
/// # Examples
///
/// ```
/// use numberlab::figurate::stella_octangula::stella_octangula_sequence;
///
/// let sequence = stella_octangula_sequence(5);
/// assert_eq!(sequence, vec![0, 1, 14, 51, 124]);
/// ```
pub fn stella_octangula_sequence(n: usize) -> Vec<u128> {
    match n {
        0 => vec![],
        _ => (0..n as u128).map(nth_stella_octangula).collect(),
    }
}
