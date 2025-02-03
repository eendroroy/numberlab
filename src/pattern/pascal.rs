/// Generates a specific row in Pascal's Triangle.
///
/// # Arguments
///
/// * `n` - The index of the row to generate.
///
/// # Returns
///
/// A vector representing the `n`-th row in Pascal's Triangle.
///
/// # Example
///
/// ```
/// use numberlab::pattern::pascal::pascal_row;
///
/// let row = pascal_row(5);
/// assert_eq!(row, vec![1, 4, 6, 4, 1]);
/// ```
pub fn pascal_row(n: usize) -> Vec<u128> {
    if n == 0 {
        return vec![];
    }

    let mut row = vec![1];
    (1..n).for_each(|i| row.push(row[i - 1] * (n as u128 - i as u128) / i as u128));
    row
}

/// Generates Pascal's Triangle up to the given number of rows.
///
/// # Arguments
///
/// * `n` - The number of rows to generate.
///
/// # Returns
///
/// A vector of vectors, where each inner vector represents a row in Pascal's Triangle.
///
/// # Example
///
/// ```
/// use numberlab::pattern::pascal::pascal_triangle;
///
/// let pascal = pascal_triangle(5);
/// assert_eq!(pascal, vec![
///     vec![1],
///     vec![1, 1],
///     vec![1, 2, 1],
///     vec![1, 3, 3, 1],
///     vec![1, 4, 6, 4, 1]
/// ]);
/// ```
pub fn pascal_triangle(n: usize) -> Vec<Vec<u128>> {
    (1..=n).map(|i| pascal_row(i)).collect()
}

/// Generates a binomial representation of Pascal's Triangle up to the given number of rows.
///
/// # Arguments
///
/// * `n` - The number of rows to generate.
///
/// # Returns
///
/// A vector representing the binomial coefficients of Pascal's Triangle up to the `n`-th row.
///
/// # Example
///
/// ```
/// use numberlab::pattern::pascal::pascal_triangle_binomial;
///
/// let binomial = pascal_triangle_binomial(5);
/// assert_eq!(binomial, vec![1, 1, 1, 1, 2, 1, 1, 3, 3, 1, 1, 4, 6, 4, 1]);
/// ```
pub fn pascal_triangle_binomial(n: usize) -> Vec<u128> {
    (1..=n).flat_map(|i| pascal_row(i)).collect()
}
