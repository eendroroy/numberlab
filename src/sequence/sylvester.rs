use num::BigUint;

/// Calculates the nth Sylvester number.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the Sylvester sequence.
///
/// # Returns
///
/// The nth Sylvester number .
///
/// # Examples
///
/// ```
/// use num::BigUint;
/// use numberlab::sequence::sylvester::nth_sylvester;
///
/// let result = nth_sylvester(5);
/// assert_eq!(result, BigUint::from(3263443u128));
/// ```
pub fn nth_sylvester(n: usize) -> BigUint {
    if n == 0 {
        BigUint::from(2u128)
    } else {
        let mth_sylvester = nth_sylvester(n - 1);
        mth_sylvester.pow(2u32) - mth_sylvester + BigUint::from(1u128)
    }
}

/// Calculates the nth Sylvester number using memoization.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the Sylvester sequence.
///
/// # Returns
///
/// The nth Sylvester number .
///
/// # Examples
///
/// ```
/// use num::BigUint;
/// use numberlab::sequence::sylvester::nth_sylvester_memoized;
///
/// let result = nth_sylvester_memoized(5);
/// assert_eq!(result, BigUint::from(3263443u128));
/// ```
pub fn nth_sylvester_memoized(n: usize) -> BigUint {
    let mut sequence = vec![BigUint::from(2u128)];
    nth_sylvester_with_memoizer(n, &mut sequence)
}

fn nth_sylvester_with_memoizer(n: usize, memoizer: &mut Vec<BigUint>) -> BigUint {
    if n < memoizer.len() {
        memoizer[n].clone()
    } else {
        let mth_sylvester = nth_sylvester_with_memoizer(n - 1, memoizer);
        let nth_sylvester = mth_sylvester.pow(2u32) - mth_sylvester + BigUint::from(1u128);
        memoizer.push(nth_sylvester);
        memoizer[n].clone()
    }
}

/// Generates a sequence of Sylvester numbers up to the given number `n`.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the number of terms in the Sylvester sequence.
///
/// # Returns
///
/// A vector containing the Sylvester sequence up to the given number `n`.
///
/// # Examples
///
/// ```
/// use num::BigUint;
/// use numberlab::sequence::sylvester::sylvester_sequence;
///
/// let sequence = sylvester_sequence(7);
/// assert_eq!(sequence, vec![
///     BigUint::from(2u128),
///     BigUint::from(3u128),
///     BigUint::from(7u128),
///     BigUint::from(43u128),
///     BigUint::from(1807u128),
///     BigUint::from(3263443u128),
///     BigUint::from(10650056950807u128)
/// ]);
/// ```
pub fn sylvester_sequence(n: usize) -> Vec<BigUint> {
    let mut sequence = vec![BigUint::from(2u128)];
    nth_sylvester_with_memoizer(n, &mut sequence);
    sequence[..n].to_vec()
}
