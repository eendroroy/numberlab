use super::*;

#[test]
fn should_panic_when_n_is_less_than_1() {
    assert!(std::panic::catch_unwind(|| nth_factorial(0)).is_err());
    assert!(std::panic::catch_unwind(|| nth_factorial_memoized(0)).is_err());
    assert!(std::panic::catch_unwind(|| factorial_sequence(0)).is_err());
}

#[test]
fn should_calculate_nth_factorial() {
    assert_eq!(nth_factorial(1), BigUint::from(1_u128));
    assert_eq!(nth_factorial(2), BigUint::from(2_u128));
    assert_eq!(nth_factorial(3), BigUint::from(6_u128));
    assert_eq!(nth_factorial(4), BigUint::from(24_u128));
    assert_eq!(nth_factorial(5), BigUint::from(120_u128));
    assert_eq!(nth_factorial(6), BigUint::from(720_u128));
    assert_eq!(nth_factorial(7), BigUint::from(5040_u128));
    assert_eq!(nth_factorial(8), BigUint::from(40320_u128));
    assert_eq!(nth_factorial(9), BigUint::from(362880_u128));
    assert_eq!(nth_factorial(10), BigUint::from(3628800_u128));
}

#[test]
fn should_calculate_nth_factorial_memoized() {
    assert_eq!(nth_factorial_memoized(1), BigUint::from(1_u128));
    assert_eq!(nth_factorial_memoized(2), BigUint::from(2_u128));
    assert_eq!(nth_factorial_memoized(3), BigUint::from(6_u128));
    assert_eq!(nth_factorial_memoized(4), BigUint::from(24_u128));
    assert_eq!(nth_factorial_memoized(5), BigUint::from(120_u128));
    assert_eq!(nth_factorial_memoized(6), BigUint::from(720_u128));
    assert_eq!(nth_factorial_memoized(7), BigUint::from(5040_u128));
    assert_eq!(nth_factorial_memoized(8), BigUint::from(40320_u128));
    assert_eq!(nth_factorial_memoized(9), BigUint::from(362880_u128));
    assert_eq!(nth_factorial_memoized(10), BigUint::from(3628800_u128));
}

#[test]
fn should_generate_factorial_sequence() {
    assert_eq!(factorial_sequence(1), vec![BigUint::from(1_u128)]);
    assert_eq!(
        factorial_sequence(10),
        vec![
            BigUint::from(1_u128),
            BigUint::from(2_u128),
            BigUint::from(6_u128),
            BigUint::from(24_u128),
            BigUint::from(120_u128),
            BigUint::from(720_u128),
            BigUint::from(5040_u128),
            BigUint::from(40320_u128),
            BigUint::from(362880_u128),
            BigUint::from(3628800_u128)
        ]
    );
}

#[test]
fn should_calculate_sum_of_factorial_sequence() {
    let series = factorial_sequence(10);
    let sum = series.iter().fold(BigUint::from(0_u128), |acc, x| acc + x);
    assert_eq!(sum, BigUint::from(4037913_u128));
}