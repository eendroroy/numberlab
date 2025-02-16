/// Macro to create a `Matrix` from a nested array.
///
/// # Example
///
/// ```
/// use numberlab::mat;
/// use numberlab::structure::matrix::Matrix;
///
/// let matrix = mat![
///     [1, 2, 3],
///     [4, 5, 6],
///     [7, 8, 9]
/// ];
///
/// assert_eq!(matrix, mat![[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
/// assert_eq!(format!("{}", matrix), "\n 1 2 3\n 4 5 6\n 7 8 9\n");
/// ```
#[macro_export]
macro_rules! mat {
    ($([$($elem:expr),* $(,)?]),* $(,)?) => {
        {
            let data = [$( [ $($elem),* ] ),*];
            Matrix::from_array(data)
        }
    };
}

#[cfg(test)]
mod mat_tests {
    use crate::structure::matrix::Matrix;

    #[test]
    fn should_create_matrix() {
        let matrix = mat![
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ];

        assert_eq!(matrix, mat![[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    }

    #[test]
    fn should_print_matrix() {
        let matrix = mat![
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ];

        assert_eq!(format!("{}", matrix), "\n 1 2 3\n 4 5 6\n 7 8 9\n");
    }
}