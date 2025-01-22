use crate::series::figurate::triangle::{nth_triangle, triangle_sequence};
use num_bigint::BigUint;

#[test]
fn should_generate_nth_triangle() {
    assert_eq!(nth_triangle(1), BigUint::from(1u128));
    assert_eq!(nth_triangle(2), BigUint::from(3u128));
    assert_eq!(nth_triangle(3), BigUint::from(6u128));
    assert_eq!(nth_triangle(4), BigUint::from(10u128));
    assert_eq!(nth_triangle(5), BigUint::from(15u128));
    assert_eq!(nth_triangle(6), BigUint::from(21u128));
    assert_eq!(nth_triangle(7), BigUint::from(28u128));
    assert_eq!(nth_triangle(8), BigUint::from(36u128));
    assert_eq!(nth_triangle(9), BigUint::from(45u128));
    assert_eq!(nth_triangle(10), BigUint::from(55u128));
}

#[test]
fn should_generate_series_of_1_items() {
    assert_eq!(triangle_sequence(1), vec![BigUint::from(1u128)]);
}

#[test]
fn should_generate_series_of_20_items() {
    assert_eq!(
        triangle_sequence(20),
        vec![
            BigUint::from(1u128),
            BigUint::from(3u128),
            BigUint::from(6u128),
            BigUint::from(10u128),
            BigUint::from(15u128),
            BigUint::from(21u128),
            BigUint::from(28u128),
            BigUint::from(36u128),
            BigUint::from(45u128),
            BigUint::from(55u128),
            BigUint::from(66u128),
            BigUint::from(78u128),
            BigUint::from(91u128),
            BigUint::from(105u128),
            BigUint::from(120u128),
            BigUint::from(136u128),
            BigUint::from(153u128),
            BigUint::from(171u128),
            BigUint::from(190u128),
            BigUint::from(210u128)
        ]
    );
}
