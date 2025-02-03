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
/// use numberlab::sequence::sylvester::nth_sylvester;
///
/// let result = nth_sylvester(5);
/// assert_eq!(result, 3263443);
/// ```
pub fn nth_sylvester(n: usize) -> u128 {
    if n == 0 {
        2u128
    } else {
        let mth_sylvester = nth_sylvester(n - 1);
        mth_sylvester.pow(2u32) - mth_sylvester + (1u128)
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
/// use numberlab::sequence::sylvester::nth_sylvester_memoized;
///
/// let result = nth_sylvester_memoized(5);
/// assert_eq!(result, 3263443);
/// ```
pub fn nth_sylvester_memoized(n: usize) -> u128 {
    let mut sequence = vec![2];
    nth_sylvester_with_memoizer(n, &mut sequence)
}

fn nth_sylvester_with_memoizer(n: usize, memoizer: &mut Vec<u128>) -> u128 {
    if n < memoizer.len() {
        memoizer[n].clone()
    } else {
        let mth_sylvester = nth_sylvester_with_memoizer(n - 1, memoizer);
        let nth_sylvester = mth_sylvester.pow(2u32) - mth_sylvester + (1u128);
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
/// use numberlab::sequence::sylvester::sylvester_sequence;
///
/// let sequence = sylvester_sequence(7);
/// assert_eq!(sequence, vec![2, 3, 7, 43, 1807, 3263443, 10650056950807]);
/// ```
pub fn sylvester_sequence(n: usize) -> Vec<u128> {
    if n == 0 {
        return vec![];
    }

    let mut sequence = vec![2];
    nth_sylvester_with_memoizer(n - 1, &mut sequence);
    sequence
}
