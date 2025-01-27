use num_bigint::BigUint;
use numberlab::sequence::sylvester::{nth_sylvester_memoized, sylvester_sequence};

#[test]
fn should_generate_nth_sylvester() {
    let mut memoizer = vec![BigUint::from(2u128)];
    assert_eq!(
        nth_sylvester_memoized(0, &mut memoizer),
        BigUint::from(2u128)
    );
    assert_eq!(
        nth_sylvester_memoized(1, &mut memoizer),
        BigUint::from(3u128)
    );
    assert_eq!(
        nth_sylvester_memoized(2, &mut memoizer),
        BigUint::from(7u128)
    );
    assert_eq!(
        nth_sylvester_memoized(3, &mut memoizer),
        BigUint::from(43u128)
    );
    assert_eq!(
        nth_sylvester_memoized(4, &mut memoizer),
        BigUint::from(1807u128)
    );
    assert_eq!(
        nth_sylvester_memoized(5, &mut memoizer),
        BigUint::from(3263443u128)
    );
    assert_eq!(
        nth_sylvester_memoized(6, &mut memoizer),
        BigUint::from(10650056950807u128)
    );
    assert_eq!(
        nth_sylvester_memoized(7, &mut memoizer),
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
