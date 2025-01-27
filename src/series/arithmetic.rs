/// Calculates the nth term of an arithmetic sequence.
///
/// # Arguments
///
/// * `a` - The first term of the sequence.
/// * `d` - The common difference between terms.
/// * `n` - The term number to calculate.
///
/// # Returns
///
/// The nth term of the arithmetic sequence.
///
/// # Examples
///
/// ```
/// use numseries::series::arithmetic::nth_arithmetic;
///
/// let a = 2.0;
/// let d = 3.0;
/// let n = 4;
/// let nth_term = nth_arithmetic(a, d, n);
/// ```
pub fn nth_arithmetic(a: f64, d: f64, n: u128) -> f64 {
    a + (d * n as f64)
}

/// Generates an arithmetic sequence.
///
/// # Arguments
///
/// * `a` - The first term of the sequence.
/// * `d` - The common difference between terms.
/// * `n` - The number of terms to generate.
///
/// # Returns
///
/// A vector containing the arithmetic sequence.
///
/// # Examples
///
/// ```
/// use numseries::series::arithmetic::arithmetic_sequence;
///
/// let a = 2.0;
/// let d = 3.0;
/// let n = 4;
/// let sequence = arithmetic_sequence(a, d, n);
/// assert_eq!(sequence, vec![2.0, 5.0, 8.0, 11.0]);
/// ```
pub fn arithmetic_sequence(a: f64, d: f64, n: u128) -> Vec<f64> {
    let mut sequence = vec![a];
    (1..n).for_each(|i| sequence.push(nth_arithmetic(a, d, i)));
    sequence
}

/// Calculates the sum of the first `n` terms of an arithmetic sequence.
///
/// # Arguments
///
/// * `a` - The first term of the sequence.
/// * `d` - The common difference between terms.
/// * `n` - The number of terms to sum.
///
/// # Returns
///
/// The sum of the first `n` terms of the arithmetic sequence.
///
/// # Examples
///
/// ```
/// use numseries::series::arithmetic::arithmetic_sum;
///
/// let a = 5.4;
/// let d = -2.9233;
/// let n = 10;
/// let sum = arithmetic_sum(a, d, n);
/// ```
pub fn arithmetic_sum(a: f64, d: f64, n: u64) -> f64 {
    ((2.0 * a + ((n as f64) - 1.0) * d) * (n as f64)) / 2.0
}

#[cfg(test)]
mod arithmetic_test;
