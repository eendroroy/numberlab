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
    let mut triangle = vec![vec![1; 1]; n];
    for i in 1..n {
        triangle[i] = vec![1; i + 1];
        for j in 1..i {
            triangle[i][j] = triangle[i - 1][j - 1] + triangle[i - 1][j];
        }
    }
    triangle
}
