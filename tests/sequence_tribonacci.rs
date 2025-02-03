use num::BigUint;
use numberlab::sequence::tribonacci::{
    nth_tribonacci, nth_tribonacci_memoized, tribonacci_sequence,
};

#[test]
fn should_generate_nth_tribonacci() {
    assert_eq!(nth_tribonacci(0), BigUint::from(0u128));
    assert_eq!(nth_tribonacci(1), BigUint::from(1u128));
    assert_eq!(nth_tribonacci(2), BigUint::from(1u128));
    assert_eq!(nth_tribonacci(3), BigUint::from(2u128));
    assert_eq!(nth_tribonacci(4), BigUint::from(4u128));
    assert_eq!(nth_tribonacci(5), BigUint::from(7u128));
    assert_eq!(nth_tribonacci(6), BigUint::from(13u128));
    assert_eq!(nth_tribonacci(7), BigUint::from(24u128));
    assert_eq!(nth_tribonacci(8,), BigUint::from(44u128));
    assert_eq!(nth_tribonacci(9), BigUint::from(81u128));
    assert_eq!(nth_tribonacci(10), BigUint::from(149u128));
}
#[test]
fn should_generate_nth_tribonacci_memoized() {
    assert_eq!(nth_tribonacci_memoized(0), BigUint::from(0u128));
    assert_eq!(nth_tribonacci_memoized(1), BigUint::from(1u128));
    assert_eq!(nth_tribonacci_memoized(2), BigUint::from(1u128));
    assert_eq!(nth_tribonacci_memoized(3), BigUint::from(2u128));
    assert_eq!(nth_tribonacci_memoized(4), BigUint::from(4u128));
    assert_eq!(nth_tribonacci_memoized(5), BigUint::from(7u128));
    assert_eq!(nth_tribonacci_memoized(6), BigUint::from(13u128));
    assert_eq!(nth_tribonacci_memoized(7), BigUint::from(24u128));
    assert_eq!(nth_tribonacci_memoized(8,), BigUint::from(44u128));
    assert_eq!(nth_tribonacci_memoized(9), BigUint::from(81u128));
    assert_eq!(nth_tribonacci_memoized(10), BigUint::from(149u128));
}

#[test]
fn should_generate_series() {
    assert_eq!(tribonacci_sequence(0), vec![]);
    assert_eq!(tribonacci_sequence(1), vec![BigUint::from(0u128)]);
    assert_eq!(
        tribonacci_sequence(2),
        vec![BigUint::from(0u128), BigUint::from(1u128)]
    );
    assert_eq!(
        tribonacci_sequence(3),
        vec![
            BigUint::from(0u128),
            BigUint::from(1u128),
            BigUint::from(1u128)
        ]
    );
    assert_eq!(
        tribonacci_sequence(10),
        vec![
            BigUint::from(0u128),
            BigUint::from(1u128),
            BigUint::from(1u128),
            BigUint::from(2u128),
            BigUint::from(4u128),
            BigUint::from(7u128),
            BigUint::from(13u128),
            BigUint::from(24u128),
            BigUint::from(44u128),
            BigUint::from(81u128)
        ]
    );
}
