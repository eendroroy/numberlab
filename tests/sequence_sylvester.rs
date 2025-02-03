use num::BigUint;
use numberlab::sequence::sylvester::{nth_sylvester, nth_sylvester_memoized, sylvester_sequence};

#[test]
fn should_generate_nth_sylvester() {
    assert_eq!(nth_sylvester(0), BigUint::from(2u128));
    assert_eq!(nth_sylvester(1), BigUint::from(3u128));
    assert_eq!(nth_sylvester(2), BigUint::from(7u128));
    assert_eq!(nth_sylvester(3), BigUint::from(43u128));
    assert_eq!(nth_sylvester(4), BigUint::from(1807u128));
    assert_eq!(nth_sylvester(5), BigUint::from(3263443u128));
    assert_eq!(nth_sylvester(6), BigUint::from(10650056950807u128));
    assert_eq!(
        nth_sylvester(7),
        BigUint::from(113423713055421844361000443u128)
    );
}

#[test]
fn should_generate_nth_sylvester_memoized() {
    assert_eq!(nth_sylvester_memoized(0), BigUint::from(2u128));
    assert_eq!(nth_sylvester_memoized(1), BigUint::from(3u128));
    assert_eq!(nth_sylvester_memoized(2), BigUint::from(7u128));
    assert_eq!(nth_sylvester_memoized(3), BigUint::from(43u128));
    assert_eq!(nth_sylvester_memoized(4), BigUint::from(1807u128));
    assert_eq!(nth_sylvester_memoized(5), BigUint::from(3263443u128));
    assert_eq!(nth_sylvester_memoized(6), BigUint::from(10650056950807u128));
    assert_eq!(
        nth_sylvester_memoized(7),
        BigUint::from(113423713055421844361000443u128)
    );
}

#[test]
fn should_generate_series_of_0_items() {
    assert_eq!(sylvester_sequence(0), vec![]);
}

#[test]
fn should_generate_series_of_1_items() {
    assert_eq!(sylvester_sequence(1), vec![BigUint::from(2u128)]);
}

#[test]
fn should_generate_series_of_10_items() {
    assert_eq!(
        sylvester_sequence(8),
        vec![
            BigUint::from(2u128),
            BigUint::from(3u128),
            BigUint::from(7u128),
            BigUint::from(43u128),
            BigUint::from(1807u128),
            BigUint::from(3263443u128),
            BigUint::from(10650056950807u128),
            BigUint::from(113423713055421844361000443u128)
        ]
    );
}
