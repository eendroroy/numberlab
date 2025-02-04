use numberlab::pattern::pascal::{pascal_row, pascal_triangle, pascal_triangle_binomial};

#[test]
fn should_generate_pascal_row() {
    assert_eq!(pascal_row(0), vec![]);
    assert_eq!(pascal_row(1), vec![1]);
    assert_eq!(pascal_row(2), vec![1, 1]);
    assert_eq!(pascal_row(3), vec![1, 2, 1]);
    assert_eq!(pascal_row(4), vec![1, 3, 3, 1]);
    assert_eq!(pascal_row(5), vec![1, 4, 6, 4, 1]);
}

#[test]
fn should_generate_pascal_triangle() {
    assert_eq!(
        pascal_triangle(16),
        vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1],
            vec![1, 6, 15, 20, 15, 6, 1],
            vec![1, 7, 21, 35, 35, 21, 7, 1],
            vec![1, 8, 28, 56, 70, 56, 28, 8, 1],
            vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1],
            vec![1, 10, 45, 120, 210, 252, 210, 120, 45, 10, 1],
            vec![1, 11, 55, 165, 330, 462, 462, 330, 165, 55, 11, 1],
            vec![1, 12, 66, 220, 495, 792, 924, 792, 495, 220, 66, 12, 1],
            vec![1, 13, 78, 286, 715, 1287, 1716, 1716, 1287, 715, 286, 78, 13, 1],
            vec![1, 14, 91, 364, 1001, 2002, 3003, 3432, 3003, 2002, 1001, 364, 91, 14, 1],
            vec![1, 15, 105, 455, 1365, 3003, 5005, 6435, 6435, 5005, 3003, 1365, 455, 105, 15, 1],
        ]
    );
}

#[test]
fn should_generate_pascal_triangle_binomial() {
    assert_eq!(
        pascal_triangle_binomial(5),
        vec![1, 1, 1, 1, 2, 1, 1, 3, 3, 1, 1, 4, 6, 4, 1, ]
    );
}
