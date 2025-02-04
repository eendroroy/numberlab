/// Generates a sequence of factorials up to the given number `n`.
///
/// # Arguments
///
/// * `n` - A positive integer representing the number of terms in the factorial sequence.
///
/// # Examples
///
/// ```
/// use numberlab::sequence::factorial::factorial_sequence;
///
/// let sequence = factorial_sequence(6);
/// assert_eq!(sequence, vec![1, 1, 2, 6, 24, 120]);
/// ```
pub fn factorial_sequence(n: usize) -> Vec<u128> {
    match n {
        0 => vec![],
        1 => vec![1],
        _ => {
            let mut sequence = vec![1u128];
            (1..n).for_each(|i| sequence.push(sequence[i - 1] * i as u128));
            sequence
        }
    }
}
