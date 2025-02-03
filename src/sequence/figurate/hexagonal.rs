use num::BigUint;
use std::ops::{Div, Mul, Sub};

/// Calculates the nth hexagonal number.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the hexagonal sequence.
///
/// # Returns
///
/// The nth hexagonal number .
///
/// # Examples
///
/// ```
/// use num::BigUint;
/// use numberlab::sequence::figurate::hexagonal::nth_hexagonal;
///
/// let result = nth_hexagonal(10);
/// assert_eq!(result, BigUint::from(190u128));
/// ```
pub fn nth_hexagonal(n: u128) -> BigUint {
    BigUint::from(n)
        .mul(BigUint::from(2u128))
        .mul(
            BigUint::from(n)
                .mul(BigUint::from(2u128))
                .sub(BigUint::from(1u128)),
        )
        .div(BigUint::from(2u128))
}

/// Generates a sequence of hexagonal numbers up to the given number `n`.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the number of terms in the hexagonal sequence.
///
/// # Returns
///
/// A vector containing the hexagonal sequence up to the given number `n`.
///
/// # Examples
///
/// ```
/// use num::BigUint;
/// use numberlab::sequence::figurate::hexagonal::hexagonal_sequence;
///
/// let sequence = hexagonal_sequence(5);
/// assert_eq!(
///     sequence,
///     vec![
///         BigUint::from(1_u128),
///         BigUint::from(6_u128),
///         BigUint::from(15_u128),
///         BigUint::from(28_u128),
///         BigUint::from(45_u128)
///     ]
/// );
/// ```
pub fn hexagonal_sequence(n: u128) -> Vec<BigUint> {
    let mut sequence = vec![];
    (1..=n).for_each(|i| sequence.push(nth_hexagonal(i)));
    sequence
}
