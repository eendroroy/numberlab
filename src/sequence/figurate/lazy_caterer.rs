use num::BigUint;
use std::ops::{Add, Div};

/// Calculates the nth lazy caterer number.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the lazy caterer sequence.
///
/// # Returns
///
/// The nth lazy caterer number .
///
/// # Examples
///
/// ```
/// use num::BigUint;
/// use numberlab::sequence::figurate::lazy_caterer::nth_lazy_caterer;
///
/// let result = nth_lazy_caterer(10);
/// assert_eq!(result, BigUint::from(56u128));
/// ```
pub fn nth_lazy_caterer(n: u128) -> BigUint {
    BigUint::from(n)
        .pow(2)
        .add(BigUint::from(n))
        .add(BigUint::from(2u128))
        .div(BigUint::from(2u128))
}

/// Generates a sequence of lazy caterer numbers up to the given number `n`.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the number of terms in the lazy caterer sequence.
///
/// # Returns
///
/// A vector containing the lazy caterer sequence up to the given number `n`.
///
/// # Examples
///
/// ```
/// use num::BigUint;
/// use numberlab::sequence::figurate::lazy_caterer::lazy_caterer_sequence;
///
/// let sequence = lazy_caterer_sequence(5);
/// assert_eq!(
///     sequence,
///     vec![
///         BigUint::from(1_u128),
///         BigUint::from(2_u128),
///         BigUint::from(4_u128),
///         BigUint::from(7_u128),
///         BigUint::from(11_u128)
///     ]
/// );
/// ```
pub fn lazy_caterer_sequence(n: u128) -> Vec<BigUint> {
    let mut sequence = vec![];
    (0..n).for_each(|i| sequence.push(nth_lazy_caterer(i)));
    sequence
}
