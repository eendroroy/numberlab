use crate::sequence::factorial::nth_factorial;

/// Calculates the number of permutations (nPr) of `n` items taken `r` at a time.
///
/// # Arguments
///
/// * `n` - The total number of items.
/// * `r` - The number of items to select.
///
/// # Panics
///
/// Panics if `n` is less than `r`.
///
/// # Examples
///
/// ```
/// use numberlab::formula::permutation::permutation;
///
/// assert_eq!(permutation(10, 2), 90);
/// assert_eq!(permutation(5, 3), 60);
/// ```
///
/// # Formula
///
/// The formula for permutations is:
/// nPr = n! / (n - r)!
///
/// This function uses the `nth_factorial` function to calculate the factorials.
///
/// # Errors
///
/// This function will panic if `n` is less than `r`.
pub fn permutation(n: u128, r: u128) -> u128 {
    if n < r {
        panic!("n must be greater than or equal to r");
    }
    nth_factorial(n) / nth_factorial(n - r)
}
