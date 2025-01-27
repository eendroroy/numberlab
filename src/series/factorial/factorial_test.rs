use super::*;

#[test]
fn test_nth_factorial() {
    assert_eq!(nth_factorial(0), BigUint::from(1u128));
    assert_eq!(nth_factorial(1), BigUint::from(1u128));
    assert_eq!(nth_factorial(2), BigUint::from(2u128));
    assert_eq!(nth_factorial(3), BigUint::from(6u128));
    assert_eq!(nth_factorial(4), BigUint::from(24u128));
    assert_eq!(nth_factorial(5), BigUint::from(120u128));
    assert_eq!(nth_factorial(6), BigUint::from(720u128));
    assert_eq!(nth_factorial(7), BigUint::from(5040u128));
    assert_eq!(nth_factorial(8), BigUint::from(40320u128));
    assert_eq!(nth_factorial(9), BigUint::from(362880u128));
    assert_eq!(nth_factorial(10), BigUint::from(3628800u128));
}

#[test]
fn test_nth_factorial_memoized() {
    assert_eq!(nth_factorial_memoized(0), BigUint::from(1u128));
    assert_eq!(nth_factorial_memoized(1), BigUint::from(1u128));
    assert_eq!(nth_factorial_memoized(2), BigUint::from(2u128));
    assert_eq!(nth_factorial_memoized(3), BigUint::from(6u128));
    assert_eq!(nth_factorial_memoized(4), BigUint::from(24u128));
    assert_eq!(nth_factorial_memoized(5), BigUint::from(120u128));
    assert_eq!(nth_factorial_memoized(6), BigUint::from(720u128));
    assert_eq!(nth_factorial_memoized(7), BigUint::from(5040u128));
    assert_eq!(nth_factorial_memoized(8), BigUint::from(40320u128));
    assert_eq!(nth_factorial_memoized(9), BigUint::from(362880u128));
    assert_eq!(nth_factorial_memoized(10), BigUint::from(3628800u128));
}

#[test]
fn test_factorial_sequence() {
    assert_eq!(factorial_sequence(0), vec![]);
    assert_eq!(factorial_sequence(1), vec![BigUint::from(1u128)]);
    assert_eq!(
        factorial_sequence(2),
        vec![BigUint::from(1u128), BigUint::from(1u128)]
    );
    assert_eq!(
        factorial_sequence(3),
        vec![BigUint::from(1u128), BigUint::from(1u128), BigUint::from(2u128)]
    );
    assert_eq!(
        factorial_sequence(10),
        vec![
            BigUint::from(1u128),
            BigUint::from(1u128),
            BigUint::from(2u128),
            BigUint::from(6u128),
            BigUint::from(24u128),
            BigUint::from(120u128),
            BigUint::from(720u128),
            BigUint::from(5040u128),
            BigUint::from(40320u128),
            BigUint::from(362880u128),
        ]
    );
}
