use crate::sequence::figurate::hexagonal::{hexagonal_sequence, nth_hexagonal};
use num_bigint::BigUint;

#[test]
fn should_generate_nth_hexagonal() {
    assert_eq!(nth_hexagonal(1), BigUint::from(1u128));
    assert_eq!(nth_hexagonal(2), BigUint::from(6u128));
    assert_eq!(nth_hexagonal(3), BigUint::from(15u128));
    assert_eq!(nth_hexagonal(4), BigUint::from(28u128));
    assert_eq!(nth_hexagonal(5), BigUint::from(45u128));
    assert_eq!(nth_hexagonal(6), BigUint::from(66u128));
    assert_eq!(nth_hexagonal(7), BigUint::from(91u128));
    assert_eq!(nth_hexagonal(8), BigUint::from(120u128));
    assert_eq!(nth_hexagonal(9), BigUint::from(153u128));
    assert_eq!(nth_hexagonal(10), BigUint::from(190u128));
}

#[test]
fn should_generate_series_of_1_items() {
    assert_eq!(hexagonal_sequence(1), vec![BigUint::from(1u128)]);
}

#[test]
fn should_generate_series_of_20_items() {
    assert_eq!(
        hexagonal_sequence(20),
        vec![
            BigUint::from(1u128),
            BigUint::from(6u128),
            BigUint::from(15u128),
            BigUint::from(28u128),
            BigUint::from(45u128),
            BigUint::from(66u128),
            BigUint::from(91u128),
            BigUint::from(120u128),
            BigUint::from(153u128),
            BigUint::from(190u128),
            BigUint::from(231u128),
            BigUint::from(276u128),
            BigUint::from(325u128),
            BigUint::from(378u128),
            BigUint::from(435u128),
            BigUint::from(496u128),
            BigUint::from(561u128),
            BigUint::from(630u128),
            BigUint::from(703u128),
            BigUint::from(780u128)
        ]
    );
}
