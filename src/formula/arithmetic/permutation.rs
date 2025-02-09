use crate::formula::arithmetic::factorial;

/// Calculates the number of permutations (`nPr`) of `n` items taken `r` at a time.
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
/// use numberlab::formula::arithmetic::permutation;
///
/// assert_eq!(permutation(10, 2), 90);
/// assert_eq!(permutation(5, 3), 60);
/// ```
pub fn permutation(n: u128, r: u128) -> u128 {
    if n < r {
        panic!("'n' ({}) must be greater than or equal to 'r' ({})", n, r);
    }
    factorial(n) / factorial(n - r)
}
