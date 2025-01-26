use super::*;

#[test]
fn test_arithmetic_sequence() {
    let sequence = arithmetic_sequence(0f64, 1f64, 10);
    assert_eq!(sequence, vec![0f64, 1f64, 2f64, 3f64, 4f64, 5f64, 6f64, 7f64, 8f64, 9f64]);

    let sequence = arithmetic_sequence(5.4, -2.92, 10);
    assert_eq!(sequence, vec![5.4, 2.48, -0.44, -3.36, -6.28, -9.2, -12.12, -15.04, -17.96, -20.88]);
}
