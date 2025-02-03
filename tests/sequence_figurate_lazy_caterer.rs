use num::BigUint;
use numberlab::sequence::figurate::lazy_caterer::{lazy_caterer_sequence, nth_lazy_caterer};

#[test]
fn should_generate_nth_lazy_caterer() {
    assert_eq!(nth_lazy_caterer(0), BigUint::from(1u128));
    assert_eq!(nth_lazy_caterer(1), BigUint::from(2u128));
    assert_eq!(nth_lazy_caterer(2), BigUint::from(4u128));
    assert_eq!(nth_lazy_caterer(3), BigUint::from(7u128));
    assert_eq!(nth_lazy_caterer(4), BigUint::from(11u128));
    assert_eq!(nth_lazy_caterer(5), BigUint::from(16u128));
    assert_eq!(nth_lazy_caterer(6), BigUint::from(22u128));
    assert_eq!(nth_lazy_caterer(7), BigUint::from(29u128));
    assert_eq!(nth_lazy_caterer(8), BigUint::from(37u128));
    assert_eq!(nth_lazy_caterer(9), BigUint::from(46u128));
    assert_eq!(nth_lazy_caterer(10), BigUint::from(56u128));
}

#[test]
fn should_generate_series_of_1_items() {
    assert_eq!(lazy_caterer_sequence(1), vec![BigUint::from(1u128)]);
}

#[test]
fn should_generate_series_of_20_items() {
    assert_eq!(
        lazy_caterer_sequence(20),
        vec![
            BigUint::from(1u128),
            BigUint::from(2u128),
            BigUint::from(4u128),
            BigUint::from(7u128),
            BigUint::from(11u128),
            BigUint::from(16u128),
            BigUint::from(22u128),
            BigUint::from(29u128),
            BigUint::from(37u128),
            BigUint::from(46u128),
            BigUint::from(56u128),
            BigUint::from(67u128),
            BigUint::from(79u128),
            BigUint::from(92u128),
            BigUint::from(106u128),
            BigUint::from(121u128),
            BigUint::from(137u128),
            BigUint::from(154u128),
            BigUint::from(172u128),
            BigUint::from(191u128)
        ]
    );
}
