use std::ops::{Add, Div, Mul, Sub};

/// Calculates the nth term of an arithmetic sequence.
///
/// # Arguments
///
/// * `a` - The first term of the sequence.
/// * `d` - The common difference between terms.
/// * `n` - The term number to calculate (must be greater than 0).
///
/// # Panics
///
/// Panics if `n` is 0.
///
/// # Examples
///
/// ```
/// use numberlab::sequence::arithmetic::nth_arithmetic;
/// let term = nth_arithmetic(1.0, 1.0, 5);
/// assert_eq!(term, 5.0);
/// ```
///
/// ```
/// use numberlab::sequence::arithmetic::nth_arithmetic;
/// let term = nth_arithmetic::<u32>(2, 3, 4);
/// assert_eq!(term, 11);
/// ```
pub fn nth_arithmetic<T>(a: T, d: T, n: u32) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + From<u32> + Copy,
{
    match n {
        0 => panic!("'n' must be greater than 0"),
        _ => a + (d * T::from(n - 1)),
    }
}

/// Generates an arithmetic sequence.
///
/// # Arguments
///
/// * `a` - The first term of the sequence.
/// * `d` - The common difference between terms.
/// * `n` - The number of terms to generate (must be greater than 0).
///
/// # Panics
///
/// Panics if `n` is 0.
///
/// # Examples
///
/// ```
/// use numberlab::sequence::arithmetic::arithmetic_sequence;
/// let sequence = arithmetic_sequence(1.0, 1.0, 5);
/// assert_eq!(sequence, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
/// ```
///
/// ```
/// use numberlab::sequence::arithmetic::arithmetic_sequence;
/// let sequence = arithmetic_sequence::<u32>(2, 3, 4);
/// assert_eq!(sequence, vec![2, 5, 8, 11]);
/// ```
pub fn arithmetic_sequence<T>(a: T, d: T, n: u32) -> Vec<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + From<u32> + Copy,
{
    match n {
        0 => panic!("'n' must be greater than 0"),
        _ => (1..=n).map(|i| nth_arithmetic(a, d, i)).collect(),
    }
}

/// Calculates the sum of the first `n` terms of an arithmetic sequence.
///
/// # Arguments
///
/// * `a` - The first term of the sequence.
/// * `d` - The common difference between terms.
/// * `n` - The number of terms to sum (must be greater than 0).
///
/// # Returns
///
/// The sum of the first `n` terms of the arithmetic sequence.
///
/// # Panics
///
/// Panics if `n` is 0.
///
/// # Examples
///
/// ```
/// use numberlab::sequence::arithmetic::arithmetic_series;
/// let sum = arithmetic_series(1.0, 1.0, 5);
/// assert_eq!(sum, 15.0);
/// ```
///
/// ```
/// use numberlab::sequence::arithmetic::arithmetic_series;
/// let sum = arithmetic_series::<u32>(2, 3, 4);
/// assert_eq!(sum, 26);
/// ```
pub fn arithmetic_series<T>(a: T, d: T, n: u32) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + From<u32> + Copy,
{
    match n {
        0 => T::from(0),
        _ => ((T::from(2) * a + T::from(n - 1) * d) * T::from(n)) / T::from(2),
    }
}
