/// Calculates the factorial of a given number `n`.
///
/// # Arguments
///
/// * `n` - A positive integer representing the number to calculate the factorial of.
///
/// # Examples
///
/// ```
/// use numberlab::formula::factorial;
///
/// let result = factorial(5);
/// assert_eq!(result, 120);
/// ```
pub fn factorial(n: u128) -> u128 {
    match n {
        0 => 1,
        _ => (1..=n).product(),
    }
}
