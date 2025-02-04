/// Computes the greatest common divisor (`GCD`) of two numbers `a` and `b`.
///
/// # Arguments
///
/// * `a` - A u128 integer.
/// * `b` - A u128 integer.
///
/// # Returns
///
/// The GCD of `a` and `b`.
///
/// # Examples
///
/// ```
/// use numberlab::formula::arithmetic::gcd;
///
/// let result = gcd(28, 35);
/// assert_eq!(result, 7);
/// ```
pub fn gcd(a: u128, b: u128) -> u128 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
