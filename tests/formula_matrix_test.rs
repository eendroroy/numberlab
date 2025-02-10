use numberlab::formula::matrix::Matrix;

#[test]
fn should_print_matrix() {
    assert_eq!(
        format!("{}", Matrix::from_array([[10, 2, 3], [3, 40, 1]])),
        "\n 10  2 3\n  3 40 1\n"
    );
    assert_eq!(
        format!("{}", Matrix::from_array([[1, 2, 3], [3, 4, 1]])),
        "\n 1 2 3\n 3 4 1\n"
    );
}

#[test]
fn should_add_matrix() {
    let matrix1 = Matrix::from_array([[10, 2, 3], [3, 40, 1]]);
    let matrix2 = Matrix::from_array([[1, 2, 3], [3, 4, 5]]);

    assert_eq!(
        format!("{}", matrix1.clone() + matrix2.clone()),
        "\n 11  4 6\n  6 44 6\n"
    );
}

#[test]
fn should_subtract_matrix() {
    let matrix1 = Matrix::from_array([[10, 2, 3], [3, 40, 1]]);
    let matrix2 = Matrix::from_array([[1, 2, 3], [3, 4, 5]]);

    assert_eq!(
        format!("{}", matrix1.clone() - matrix2.clone()),
        "\n 9  0  0\n 0 36 -4\n"
    );
}

#[test]
fn should_multiply_matrix() {
    let matrix1 = Matrix::from_array([[10, 2, 3], [3, 40, 1]]);
    let matrix2 = Matrix::from_array([[1, 2], [3, 4], [5, 6]]);

    assert_eq!(
        format!("{}", matrix1.clone() * matrix2.clone()),
        "\n  31  46\n 128 172\n"
    );
}

#[test]
fn should_divide_matrix() {
    let matrix1 = Matrix::from_array([[10, 2, 3], [3, 40, 1]]);
    let matrix2 = Matrix::from_array([[1, 2, 3], [3, 4, 5]]);

    assert_eq!(
        format!("{}", matrix1.clone() / matrix2.clone()),
        "\n 10  1 1\n  1 10 0\n"
    );
}

#[test]
fn should_return_index() {
    let matrix = Matrix::from_array([[10.0, 2.0, 3.0], [3.0, 40.0, 1.0]]);

    assert_eq!(matrix[(0, 0)], 10.0);
    assert_eq!(matrix[(0, 1)], 2.0);
    assert_eq!(matrix[(0, 2)], 3.0);
    assert_eq!(matrix[(1, 0)], 3.0);
    assert_eq!(matrix[(1, 1)], 40.0);
    assert_eq!(matrix[(1, 2)], 1.0);

    let matrix = &mut Matrix::from_array(
        [
            [10, 2, 3, 3],
            [3, 40, 1, 6],
            [31, 40, 10, 5],
            [3, 4, 1, 6],
        ]
    );

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
