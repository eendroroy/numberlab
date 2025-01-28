use num_bigint::BigUint;

/// Calculates the nth Lucas number.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the Lucas sequence.
///
/// # Returns
///
/// The nth Lucas number as a `BigUint`.
///
/// # Examples
///
/// ```
/// use num_bigint::BigUint;
/// use numberlab::sequence::lucas::nth_lucas;
///
/// let result = nth_lucas(10);
/// assert_eq!(result, BigUint::from(123u128));
/// ```
pub fn nth_lucas(n: usize) -> BigUint {
    match n {
        0 => BigUint::from(2u128),
        1 => BigUint::from(1u128),
        _ => nth_lucas(n - 1) + nth_lucas(n - 2),
    }
}

/// Calculates the nth Lucas number using memoization.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the Lucas sequence.
///
/// # Returns
///
/// The nth Lucas number as a `BigUint`.
///
/// # Examples
///
/// ```
/// use num_bigint::BigUint;
/// use numberlab::sequence::lucas::nth_lucas_memoized;
///
/// let result = nth_lucas_memoized(10);
/// assert_eq!(result, BigUint::from(123u128));
/// ```
pub fn nth_lucas_memoized(n: usize) -> BigUint {
    let mut memoizer = vec![BigUint::from(2u128), BigUint::from(1u128)];
    nth_lucas_with_memoizer(n, &mut memoizer)
}

fn nth_lucas_with_memoizer(n: usize, memoizer: &mut Vec<BigUint>) -> BigUint {
    if n < memoizer.len() {
        memoizer[n].clone()
    } else {
        let lth_lucas = nth_lucas_with_memoizer(n - 1, memoizer);
        let mth_lucas = nth_lucas_with_memoizer(n - 2, memoizer);
        let nth_lucas = lth_lucas + mth_lucas;
        memoizer.push(nth_lucas);
        memoizer[n].clone()
    }
}

/// Generates a sequence of Lucas numbers up to the given number `n`.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the number of terms in the Lucas sequence.
///
/// # Returns
///
/// A vector containing the Lucas sequence up to the given number `n`.
///
/// # Examples
///
/// ```
/// use num_bigint::BigUint;
/// use numberlab::sequence::lucas::lucas_sequence;
///
/// let sequence = lucas_sequence(5);
/// assert_eq!(
///     sequence,
///     vec![
///         BigUint::from(2_u128),
///         BigUint::from(1_u128),
///         BigUint::from(3_u128),
///         BigUint::from(4_u128),
///         BigUint::from(7_u128)
///     ]
/// );
/// ```
pub fn lucas_sequence(n: usize) -> Vec<BigUint> {
    let mut sequence = vec![BigUint::from(2u128), BigUint::from(1u128)];
    (2..n).for_each(|i| sequence.push(&sequence[i - 1] + &sequence[i - 2]));
    sequence
}
