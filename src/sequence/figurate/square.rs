use num::BigUint;

/// Calculates the nth square number.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the square sequence.
///
/// # Returns
///
/// The nth square number as a `BigUint`.
///
/// # Examples
///
/// ```
/// use num::BigUint;
/// use numberlab::sequence::figurate::square::nth_square;
///
/// let result = nth_square(10);
/// assert_eq!(result, BigUint::from(100u128));
/// ```
pub fn nth_square(n: u128) -> BigUint {
    BigUint::from(n).pow(2)
}

/// Generates a sequence of square numbers up to the given number `n`.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the number of terms in the square sequence.
///
/// # Returns
///
/// A vector containing the square sequence up to the given number `n`.
///
/// # Examples
///
/// ```
/// use num::BigUint;
/// use numberlab::sequence::figurate::square::square_sequence;
///
/// let sequence = square_sequence(5);
/// assert_eq!(
///     sequence,
///     vec![
///         BigUint::from(1_u128),
///         BigUint::from(4_u128),
///         BigUint::from(9_u128),
///         BigUint::from(16_u128),
///         BigUint::from(25_u128)
///     ]
/// );
/// ```
pub fn square_sequence(n: u128) -> Vec<BigUint> {
    let mut sequence = vec![];
    (1..=n).for_each(|i| sequence.push(nth_square(i)));
    sequence
}
