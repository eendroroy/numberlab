use num::BigUint;
use std::ops::{Div, Mul, Sub};

/// Calculates the nth pentagonal number.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the pentagonal sequence.
///
/// # Returns
///
/// The nth pentagonal number as a `BigUint`.
///
/// # Examples
///
/// ```
/// use num::BigUint;
/// use numberlab::sequence::figurate::pentagonal::nth_pentagonal;
///
/// let result = nth_pentagonal(10);
/// assert_eq!(result, BigUint::from(145u128));
/// ```
pub fn nth_pentagonal(n: u128) -> BigUint {
    BigUint::from(n)
        .mul(BigUint::from(n).mul(BigUint::from(3u128)).sub(BigUint::from(1u128)))
        .div(BigUint::from(2u128))
}

/// Generates a sequence of pentagonal numbers up to the given number `n`.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the number of terms in the pentagonal sequence.
///
/// # Returns
///
/// A vector containing the pentagonal sequence up to the given number `n`.
///
/// # Examples
///
/// ```
/// use num::BigUint;
/// use numberlab::sequence::figurate::pentagonal::pentagonal_sequence;
///
/// let sequence = pentagonal_sequence(5);
/// assert_eq!(
///     sequence,
///     vec![
///         BigUint::from(1_u128),
///         BigUint::from(5_u128),
///         BigUint::from(12_u128),
///         BigUint::from(22_u128),
///         BigUint::from(35_u128)
///     ]
/// );
/// ```
pub fn pentagonal_sequence(n: u128) -> Vec<BigUint> {
    let mut sequence = vec![];
    (1..=n).for_each(|i| sequence.push(nth_pentagonal(i)));
    sequence
}
