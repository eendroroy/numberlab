use num::BigUint;
use numberlab::sequence::fibonacci::{
    fibonacci_sequence, fibonacci_series, nth_fibonacci, nth_fibonacci_memoized,
};

#[test]
fn should_generate_nth_fibonacci() {
    assert_eq!(nth_fibonacci(0), BigUint::from(0u128));
    assert_eq!(nth_fibonacci(1), BigUint::from(1u128));
    assert_eq!(nth_fibonacci(2), BigUint::from(1u128));
    assert_eq!(nth_fibonacci(3), BigUint::from(2u128));
    assert_eq!(nth_fibonacci(4), BigUint::from(3u128));
    assert_eq!(nth_fibonacci(5), BigUint::from(5u128));
    assert_eq!(nth_fibonacci(10), BigUint::from(55u128));
}

#[test]
fn should_generate_nth_fibonacci_memoized() {
    assert_eq!(nth_fibonacci_memoized(0), BigUint::from(0u128));
    assert_eq!(nth_fibonacci_memoized(1), BigUint::from(1u128));
    assert_eq!(nth_fibonacci_memoized(2), BigUint::from(1u128));
    assert_eq!(nth_fibonacci_memoized(3), BigUint::from(2u128));
    assert_eq!(nth_fibonacci_memoized(4), BigUint::from(3u128));
    assert_eq!(nth_fibonacci_memoized(5), BigUint::from(5u128));
    assert_eq!(nth_fibonacci_memoized(10), BigUint::from(55u128));
}

#[test]
fn should_generate_sequence() {
    assert_eq!(fibonacci_sequence(0), vec![]);
    assert_eq!(fibonacci_sequence(1), vec![BigUint::from(0u128)]);
    assert_eq!(
        fibonacci_sequence(2),
        vec![BigUint::from(0u128), BigUint::from(1u128)]
    );
    assert_eq!(
        fibonacci_sequence(10),
        vec![
            BigUint::from(0u128),
            BigUint::from(1u128),
            BigUint::from(1u128),
            BigUint::from(2u128),
            BigUint::from(3u128),
            BigUint::from(5u128),
            BigUint::from(8u128),
            BigUint::from(13u128),
            BigUint::from(21u128),
            BigUint::from(34u128),
        ]
    );
}

#[test]
fn should_generate_series() {
    assert_eq!(fibonacci_series(0), BigUint::from(0u128));
    assert_eq!(fibonacci_series(1), BigUint::from(0u128));
    assert_eq!(fibonacci_series(2), BigUint::from(1u128));
    assert_eq!(fibonacci_series(3), BigUint::from(2u128));
    assert_eq!(fibonacci_series(4), BigUint::from(4u128));
    assert_eq!(fibonacci_series(5), BigUint::from(7u128));
    assert_eq!(fibonacci_series(10), BigUint::from(88u128));
}
