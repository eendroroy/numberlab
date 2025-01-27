use super::*;

fn round_to_precision(number: f64, precision: u8) -> f64 {
    (number * 10_f64.powf(precision as f64)).round() / 10.0_f64.powf(precision as f64)
}

#[test]
fn should_panic_when_n_is_less_than_1() {
    assert!(std::panic::catch_unwind(|| nth_arithmetic(1.0, 1.0, 0)).is_err());
    assert!(std::panic::catch_unwind(|| arithmetic_sequence(1.0, 1.0, 0)).is_err());
    assert!(std::panic::catch_unwind(|| arithmetic_sum(1.0, 1.0, 0)).is_err());
}

#[test]
fn should_calculate_nth_arithmetic() {
    assert_eq!(nth_arithmetic(0.0, 1.0, 1), 0.0);
    assert_eq!(nth_arithmetic(0.0, 1.0, 2), 1.0);
    assert_eq!(nth_arithmetic(0.0, 1.0, 3), 2.0);
    assert_eq!(nth_arithmetic(0.0, 1.0, 4), 3.0);
    assert_eq!(nth_arithmetic(0.0, 1.0, 5), 4.0);
    assert_eq!(nth_arithmetic(0.0, 1.0, 6), 5.0);
    assert_eq!(nth_arithmetic(0.0, 1.0, 7), 6.0);
    assert_eq!(nth_arithmetic(0.0, 1.0, 8), 7.0);
    assert_eq!(nth_arithmetic(0.0, 1.0, 9), 8.0);
}

#[test]
fn should_generate_arithmetic_sequence() {
    let sequence = arithmetic_sequence(0.0, 1.0, 10);
    assert_eq!(
        sequence,
        vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
    );

    let sequence = arithmetic_sequence(5.4, -2.92, 10)
        .iter()
        .map(|n| round_to_precision(*n, 2))
        .collect::<Vec<f64>>();
    assert_eq!(
        sequence,
        vec![5.4, 2.48, -0.44, -3.36, -6.28, -9.2, -12.12, -15.04, -17.96, -20.88]
    );
}

#[test]
fn should_calculate_arithmetic_sum() {
    assert_eq!(arithmetic_sum(0.0, 1.0, 1), 0.0);
    assert_eq!(arithmetic_sum(0.0, 1.0, 2), 1.0);
    assert_eq!(arithmetic_sum(0.0, 1.0, 3), 3.0);
    assert_eq!(arithmetic_sum(0.0, 1.0, 4), 6.0);
    assert_eq!(arithmetic_sum(0.0, 1.0, 5), 10.0);
    assert_eq!(arithmetic_sum(0.0, 1.0, 6), 15.0);
    assert_eq!(arithmetic_sum(0.0, 1.0, 7), 21.0);
    assert_eq!(arithmetic_sum(0.0, 1.0, 8), 28.0);
    assert_eq!(arithmetic_sum(0.0, 1.0, 9), 36.0);
    assert_eq!(arithmetic_sum(0.0, 1.0, 10), 45.0);
}
