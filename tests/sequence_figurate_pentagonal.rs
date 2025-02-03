use num::BigUint;
use numberlab::sequence::figurate::pentagonal::{nth_pentagonal, pentagonal_sequence};

#[test]
fn should_generate_nth_pentagonal() {
    assert_eq!(nth_pentagonal(1), BigUint::from(1u128));
    assert_eq!(nth_pentagonal(2), BigUint::from(5u128));
    assert_eq!(nth_pentagonal(3), BigUint::from(12u128));
    assert_eq!(nth_pentagonal(4), BigUint::from(22u128));
    assert_eq!(nth_pentagonal(5), BigUint::from(35u128));
    assert_eq!(nth_pentagonal(6), BigUint::from(51u128));
    assert_eq!(nth_pentagonal(7), BigUint::from(70u128));
    assert_eq!(nth_pentagonal(8), BigUint::from(92u128));
    assert_eq!(nth_pentagonal(9), BigUint::from(117u128));
    assert_eq!(nth_pentagonal(10), BigUint::from(145u128));
}

#[test]
fn should_generate_series_of_1_items() {
    assert_eq!(pentagonal_sequence(1), vec![BigUint::from(1u128)]);
}

#[test]
fn should_generate_series_of_20_items() {
    assert_eq!(
        pentagonal_sequence(20),
        vec![
            BigUint::from(1u128),
            BigUint::from(5u128),
            BigUint::from(12u128),
            BigUint::from(22u128),
            BigUint::from(35u128),
            BigUint::from(51u128),
            BigUint::from(70u128),
            BigUint::from(92u128),
            BigUint::from(117u128),
            BigUint::from(145u128),
            BigUint::from(176u128),
            BigUint::from(210u128),
            BigUint::from(247u128),
            BigUint::from(287u128),
            BigUint::from(330u128),
            BigUint::from(376u128),
            BigUint::from(425u128),
            BigUint::from(477u128),
            BigUint::from(532u128),
            BigUint::from(590u128)
        ]
    );
}
