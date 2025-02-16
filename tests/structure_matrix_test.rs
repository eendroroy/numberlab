use numberlab::mat;
use numberlab::structure::matrix::{
    identity, lower_triangular, transpose, upper_triangular, Matrix,
};

#[test]
fn should_create_matrix_of_different_data_type() {
    assert_eq!(format!("{}", mat![[1i8, 2]]), "\n 1 2\n");
    assert_eq!(format!("{}", mat![[1i16, 2]]), "\n 1 2\n");
    assert_eq!(format!("{}", mat![[1i32, 2]]), "\n 1 2\n");
    assert_eq!(format!("{}", mat![[1i64, 2]]), "\n 1 2\n");
    assert_eq!(format!("{}", mat![[1i128, 2]]), "\n 1 2\n");
    assert_eq!(format!("{}", mat![[1isize, 2]]), "\n 1 2\n");
    assert_eq!(format!("{}", mat![[1.0f32, 2.0]]), "\n 1 2\n");
    assert_eq!(format!("{}", mat![[1.0f64, 2.0]]), "\n 1 2\n");
}

#[test]
fn should_print_matrix() {
    assert_eq!(
        format!("{}", mat![[10, 2, 3], [3, 40, 1]]),
        "\n 10  2 3\n  3 40 1\n"
    );
    assert_eq!(
        format!("{}", mat![[1, 2, 3], [3, 4, 1]]),
        "\n 1 2 3\n 3 4 1\n"
    );
}

#[test]
fn should_add_matrix() {
    let matrix1 = mat![[10, 2, 3], [3, 40, 1]];
    let matrix2 = mat![[1, 2, 3], [3, 4, 5]];

    assert_eq!(
        format!("{}", matrix1.clone() + matrix2.clone()),
        "\n 11  4 6\n  6 44 6\n"
    );
}

#[test]
fn should_subtract_matrix() {
    let matrix1 = mat![[10, 2, 3], [3, 40, 1]];
    let matrix2 = mat![[1, 2, 3], [3, 4, 5]];

    assert_eq!(
        format!("{}", matrix1.clone() - matrix2.clone()),
        "\n 9  0  0\n 0 36 -4\n"
    );
}

#[test]
fn should_multiply_matrix() {
    let matrix1 = mat![[10, 2, 3], [3, 40, 1]];
    let matrix2 = mat![[1, 2], [3, 4], [5, 6]];

    assert_eq!(
        format!("{}", matrix1.clone() * matrix2.clone()),
        "\n  31  46\n 128 172\n"
    );
}

#[test]
fn should_return_index() {
    let matrix = mat![[10.0, 2.0, 3.0], [3.0, 40.0, 1.0]];

    assert_eq!(matrix[(0, 0)], 10.0);
    assert_eq!(matrix[(0, 1)], 2.0);
    assert_eq!(matrix[(0, 2)], 3.0);
    assert_eq!(matrix[(1, 0)], 3.0);
    assert_eq!(matrix[(1, 1)], 40.0);
    assert_eq!(matrix[(1, 2)], 1.0);

    let matrix = &mut mat![[10, 2, 3, 3], [3, 40, 1, 6], [31, 40, 10, 5], [3, 4, 1, 6],];

    assert_eq!(matrix[(0, 0)], 10);
    assert_eq!(matrix[(0, 1)], 2);
    assert_eq!(matrix[(0, 2)], 3);
    assert_eq!(matrix[(0, 3)], 3);
    assert_eq!(matrix[(1, 0)], 3);
    assert_eq!(matrix[(1, 1)], 40);
    assert_eq!(matrix[(1, 2)], 1);
    assert_eq!(matrix[(1, 3)], 6);
    assert_eq!(matrix[(2, 0)], 31);
    assert_eq!(matrix[(2, 1)], 40);
    assert_eq!(matrix[(2, 2)], 10);
    assert_eq!(matrix[(2, 3)], 5);
    assert_eq!(matrix[(3, 0)], 3);
    assert_eq!(matrix[(3, 1)], 4);
    assert_eq!(matrix[(3, 2)], 1);
    assert_eq!(matrix[(3, 3)], 6);

    matrix[(2, 1)] = 4;
    matrix[(2, 2)] = 1;

    assert_eq!(matrix[(2, 1)], 4);
    assert_eq!(matrix[(2, 2)], 1);
}

#[test]
fn should_generate_identity_matrix() {
    assert_eq!(identity::<i32, 1>(), mat![[1]]);
    assert_eq!(identity::<i32, 2>(), mat![[1, 0], [0, 1]]);
    assert_eq!(
        identity::<i32, 10>(),
        mat![
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        ]
    );
}

#[test]
fn should_transpose_matrix() {
    let matrix1 = mat![[10, 2, 3], [3, 40, 1]];
    let matrix2 = mat![[1, 2, 3], [3, 4, 5]];
    let matrix3 = mat![[1, 2], [3, 4], [5, 6]];

    assert_eq!(transpose(matrix1), mat![[10, 3], [2, 40], [3, 1]]);
    assert_eq!(transpose(matrix2), mat![[1, 3], [2, 4], [3, 5]]);
    assert_eq!(transpose(matrix3), mat![[1, 3, 5], [2, 4, 6]]);
}

#[test]
fn should_return_row() {
    let matrix = mat![[10, 2, 3], [3, 40, 1]];

    assert_eq!(matrix.row(0), [10, 2, 3]);
    assert_eq!(matrix.row(1), [3, 40, 1]);
}

#[test]
fn should_return_column() {
    let matrix = mat![[10, 2, 3], [3, 40, 1]];

    assert_eq!(matrix.col(0), [10, 3]);
    assert_eq!(matrix.col(1), [2, 40]);
    assert_eq!(matrix.col(2), [3, 1]);
}

#[test]
fn should_add_scalar_to_matrix() {
    let matrix = mat![[10, 2, 3], [3, 40, 1]];

    assert_eq!(
        format!("{}", matrix.clone() + 2),
        "\n 12  4 5\
         \n  5 42 3\n"
    );
}

#[test]
fn should_subtract_scalar_from_matrix() {
    let matrix = mat![[10, 2, 3], [3, 40, 1]];

    assert_eq!(
        format!("{}", matrix.clone() - 2),
        "\n 8  0  1\
         \n 1 38 -1\n"
    );
}

#[test]
fn should_multiply_scalar_to_matrix() {
    let matrix = mat![[10, 2, 3], [3, 40, 1]];

    assert_eq!(
        format!("{}", matrix.clone() * 2),
        "\n 20  4 6\
         \n  6 80 2\n"
    );
}

#[test]
fn should_divide_matrix_by_scalar() {
    let matrix = mat![[10.0, 2.0, 3.0], [3.0, 40.0, 1.0]];

    assert_eq!(
        format!("{}", matrix.clone() / 2.0),
        "\n   5  1 1.5\
         \n 1.5 20 0.5\n"
    );
}

#[test]
fn should_check_if_matrix_is_square() {
    let matrix1 = mat![[10, 2, 3], [3, 40, 1]];
    let matrix2 = mat![[1, 2, 3], [3, 4, 5], [5, 6, 7]];

    assert_eq!(matrix1.is_square(), false);
    assert_eq!(matrix2.is_square(), true);
}

#[test]
fn should_check_if_matrix_is_identity() {
    let matrix1 = mat![[10, 2, 3], [3, 40, 1]];
    let matrix2 = mat![[1, 0], [0, 1]];
    let matrix3 = mat![[10, 2, 3], [3, 40, 1], [3, 40, 1]];
    let matrix4 = mat![[1, 1, 3], [3, 40, 1], [3, 40, 1]];

    assert_eq!(matrix1.is_identity(), false);
    assert_eq!(matrix2.is_identity(), true);
    assert_eq!(matrix3.is_identity(), false);
    assert_eq!(matrix4.is_identity(), false);
}

#[test]
fn should_check_if_matrix_is_diagonal() {
    let matrix1 = mat![[10, 2, 3], [3, 40, 1]];
    let matrix2 = mat![[1, 0, 0], [0, 4, 0], [0, 0, 7]];
    let matrix3 = mat![[10, 2, 3], [3, 40, 1], [3, 40, 1]];
    let matrix4 = mat![[1, 0, 0], [1, 4, 0], [0, 0, 7]];
    let matrix5 = mat![[1, 0, 0], [0, 0, 0], [0, 0, 7]];

    assert_eq!(matrix1.is_diagonal(), false);
    assert_eq!(matrix2.is_diagonal(), true);
    assert_eq!(matrix3.is_diagonal(), false);
    assert_eq!(matrix4.is_diagonal(), false);
    assert_eq!(matrix5.is_diagonal(), false);
}

#[test]
fn should_check_if_matrix_is_symmetric() {
    let matrix1 = mat![[1, 2, 3], [2, 4, 5], [3, 5, 6]];
    let matrix2 = mat![[1, 2, 3], [3, 4, 5], [5, 6, 7]];

    assert_eq!(matrix1.is_symmetric(), true);
    assert_eq!(matrix2.is_symmetric(), false);
}

#[test]
fn should_check_if_matrix_is_upper_triangular() {
    let matrix1 = mat![[10, 2, 3], [3, 40, 1]];
    let matrix2 = mat![[1, 2, 3], [0, 4, 5], [0, 0, 7]];
    let matrix3 = mat![[10, 2, 3], [3, 40, 1], [3, 40, 1]];

    assert_eq!(matrix1.is_upper_triangular(), false);
    assert_eq!(matrix2.is_upper_triangular(), true);
    assert_eq!(matrix3.is_upper_triangular(), false);
}

#[test]
fn should_check_if_matrix_is_lower_triangular() {
    let matrix1 = mat![[10, 2, 3], [3, 40, 1]];
    let matrix2 = mat![[1, 0, 0], [3, 4, 0], [5, 6, 7]];
    let matrix3 = mat![[10, 2, 3], [3, 40, 1], [3, 40, 1]];

    assert_eq!(matrix1.is_lower_triangular(), false);
    assert_eq!(matrix2.is_lower_triangular(), true);
    assert_eq!(matrix3.is_lower_triangular(), false);
}

#[test]
fn should_return_upper_triangular_matrix() {
    let matrix = mat![[1, 2, 3], [3, 4, 5], [5, 6, 7]];

    assert_eq!(
        upper_triangular(matrix),
        mat![[1, 2, 3], [0, 4, 5], [0, 0, 7]]
    );
}

#[test]
fn should_return_lower_triangular_matrix() {
    let matrix = mat![[1, 2, 3], [3, 4, 5], [5, 6, 7]];

    assert_eq!(
        lower_triangular(matrix),
        mat![[1, 0, 0], [3, 4, 0], [5, 6, 7]]
    );
}
