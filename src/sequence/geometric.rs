/// Calculates the nth term of a geometric sequence.
///
/// # Arguments
///
/// * `a` - The first term of the sequence.
/// * `r` - The common ratio of the sequence.
/// * `n` - The term number to calculate (must be greater than 0).
///
/// # Returns
///
/// The nth term of the geometric sequence.
///
/// # Panics
///
/// Panics if `n` is 0.
///
/// # Examples
///
/// ```
/// use numberlab::sequence::geometric::nth_geometric;
/// let term = nth_geometric(1.12, 2.23, 3);
/// assert_eq!(term, 1.12 * 2.23 * 2.23);
/// ```
pub fn nth_geometric(a: f64, r: f64, n: u64) -> f64 {
    match n {
        0 => panic!("'n' must be greater than 0"),
        1 => a,
        _ => a * r.powf(n as f64 - 1.0),
    }
}

/// Generates a geometric sequence.
///
/// # Arguments
///
/// * `a` - The first term of the sequence.
/// * `r` - The common ratio of the sequence.
/// * `n` - The number of terms to generate.
///
/// # Returns
///
/// A vector containing the geometric sequence.
///
/// # Examples
///
/// ```
/// use numberlab::sequence::geometric::geometric_sequence;
/// let sequence = geometric_sequence(1.12, 2.23, 3);
/// assert_eq!(sequence, vec![1.12, 1.12 * 2.23, 1.12 * 2.23 * 2.23]);
/// ```
pub fn geometric_sequence(a: f64, r: f64, n: u64) -> Vec<f64> {
    let mut sequence = vec![];
    for i in 1..=n {
        let ith = nth_geometric(a, r, i);

        sequence.push(ith);
    }
    sequence
}

/// Calculates the sum of the first `n` terms of a geometric series.
///
/// # Arguments
///
/// * `a` - The first term of the series.
/// * `r` - The common ratio of the series.
/// * `n` - The number of terms to sum.
///
/// # Returns
///
/// The sum of the first `n` terms of the geometric series.
///
/// # Examples
///
/// ```
/// use numberlab::sequence::geometric::geometric_series;
/// let sum = geometric_series(1.12, 2.23, 3);
/// assert_eq!(sum, 1.12 * (1.0 + 2.23 + 2.23 * 2.23));
/// ```
pub fn geometric_series(a: f64, r: f64, n: u64) -> f64 {
    match (n, r) {
        (0, _) => 0.0,
        (1, _) => a,
        (_, 1.0) => a * n as f64,
        _ => a * ((r.powf(n as f64) - 1f64) / (r - 1f64)),
    }
}

#[cfg(test)]
mod geometric_test;
