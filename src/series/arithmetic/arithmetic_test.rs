use super::*;

#[test]
fn should_calculate_nth_arithmetic() {
    assert_eq!(nth_arithmetic(0f64, 1f64, 0), 0f64);
    assert_eq!(nth_arithmetic(0f64, 1f64, 1), 1f64);
    assert_eq!(nth_arithmetic(0f64, 1f64, 2), 2f64);
    assert_eq!(nth_arithmetic(0f64, 1f64, 3), 3f64);
    assert_eq!(nth_arithmetic(0f64, 1f64, 4), 4f64);
    assert_eq!(nth_arithmetic(0f64, 1f64, 5), 5f64);
    assert_eq!(nth_arithmetic(0f64, 1f64, 6), 6f64);
    assert_eq!(nth_arithmetic(0f64, 1f64, 7), 7f64);
    assert_eq!(nth_arithmetic(0f64, 1f64, 8), 8f64);
    assert_eq!(nth_arithmetic(0f64, 1f64, 9), 9f64);
}

#[test]
fn should_generate_arithmetic_sequence() {
    let sequence = arithmetic_sequence(0f64, 1f64, 10);
    assert_eq!(
        sequence,
        vec![0f64, 1f64, 2f64, 3f64, 4f64, 5f64, 6f64, 7f64, 8f64, 9f64]
    );

    let sequence = arithmetic_sequence(5.4, -2.92, 10);
    assert_eq!(
        sequence,
        vec![5.4, 2.48, -0.44, -3.36, -6.28, -9.2, -12.12, -15.04, -17.96, -20.88]
    );
}

#[test]
fn should_calculate_arithmetic_sum() {
    assert_eq!(arithmetic_sum(0f64, 1f64, 1), 0f64);
    assert_eq!(arithmetic_sum(0f64, 1f64, 2), 1f64);
    assert_eq!(arithmetic_sum(0f64, 1f64, 3), 3f64);
    assert_eq!(arithmetic_sum(0f64, 1f64, 4), 6f64);
    assert_eq!(arithmetic_sum(0f64, 1f64, 5), 10f64);
    assert_eq!(arithmetic_sum(0f64, 1f64, 6), 15f64);
    assert_eq!(arithmetic_sum(0f64, 1f64, 7), 21f64);
    assert_eq!(arithmetic_sum(0f64, 1f64, 8), 28f64);
    assert_eq!(arithmetic_sum(0f64, 1f64, 9), 36f64);
    assert_eq!(arithmetic_sum(0f64, 1f64, 10), 45f64);
}
