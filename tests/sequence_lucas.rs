use num_bigint::BigUint;
use numberlab::sequence::lucas::{lucas_sequence, nth_lucas, nth_lucas_memoized};

#[test]
fn should_generate_nth_lucas() {
    assert_eq!(nth_lucas(0), BigUint::from(2u128));
    assert_eq!(nth_lucas(1), BigUint::from(1u128));
    assert_eq!(nth_lucas(7), BigUint::from(29u128));
    assert_eq!(nth_lucas(8), BigUint::from(47u128));
    assert_eq!(nth_lucas(9), BigUint::from(76u128));
    assert_eq!(nth_lucas(10), BigUint::from(123u128));
}

#[test]
fn should_generate_nth_lucas_with_memoization() {
    assert_eq!(nth_lucas_memoized(0), BigUint::from(2u128));
    assert_eq!(nth_lucas_memoized(1), BigUint::from(1u128));
    assert_eq!(nth_lucas_memoized(2), BigUint::from(3u128));
    assert_eq!(nth_lucas_memoized(3), BigUint::from(4u128));
    assert_eq!(nth_lucas_memoized(4), BigUint::from(7u128));
    assert_eq!(nth_lucas_memoized(5), BigUint::from(11u128));
    assert_eq!(nth_lucas_memoized(6), BigUint::from(18u128));
    assert_eq!(nth_lucas_memoized(7), BigUint::from(29u128));
    assert_eq!(nth_lucas_memoized(8), BigUint::from(47u128));
    assert_eq!(nth_lucas_memoized(9), BigUint::from(76u128));
    assert_eq!(nth_lucas_memoized(10), BigUint::from(123u128));
}

#[test]
fn should_generate_series_of_0_items() {
    assert_eq!(lucas_sequence(0), vec![]);
}

#[test]
fn should_generate_series_of_1_items() {
    assert_eq!(lucas_sequence(1), vec![BigUint::from(2u128)]);
}

#[test]
fn should_generate_series_of_10_items() {
    assert_eq!(
        lucas_sequence(10),
        vec![
            BigUint::from(2u128),
            BigUint::from(1u128),
            BigUint::from(3u128),
            BigUint::from(4u128),
            BigUint::from(7u128),
            BigUint::from(11u128),
            BigUint::from(18u128),
            BigUint::from(29u128),
            BigUint::from(47u128),
            BigUint::from(76u128)
        ]
    );
}
