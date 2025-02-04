use crate::formula::gcd::gcd;

/// Computes the least common multiple (LCM) of two numbers `a` and `b`.
///
/// # Arguments
///
/// * `a` - A u128 integer.
/// * `b` - A u128 integer.
///
/// # Returns
///
/// The LCM of `a` and `b`.
///
/// # Examples
///
/// ```
/// use numberlab::formula::lcm;
///
/// let result = lcm(28, 35);
/// assert_eq!(result, 140);
/// ```
pub fn lcm(a: u128, b: u128) -> u128 {
    match (a, b) {
        (0, 0) => 0,
        _ => a * b / gcd(a, b),
    }
}
