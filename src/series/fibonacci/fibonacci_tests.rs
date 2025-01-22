use super::*;

#[test]
fn should_generate_nth_fibonacci() {
    let mut memoizer = vec![BigUint::from(0u128), BigUint::from(1u128)];
    assert_eq!(
        nth_fibonacci_memoized(0, &mut memoizer),
        BigUint::from(0u128)
    );
    assert_eq!(
        nth_fibonacci_memoized(1, &mut memoizer),
        BigUint::from(1u128)
    );
    assert_eq!(
        nth_fibonacci_memoized(2, &mut memoizer),
        BigUint::from(1u128)
    );
    assert_eq!(
        nth_fibonacci_memoized(3, &mut memoizer),
        BigUint::from(2u128)
    );
    assert_eq!(
        nth_fibonacci_memoized(4, &mut memoizer),
        BigUint::from(3u128)
    );
    assert_eq!(
        nth_fibonacci_memoized(5, &mut memoizer),
        BigUint::from(5u128)
    );
    assert_eq!(
        nth_fibonacci_memoized(6, &mut memoizer),
        BigUint::from(8u128)
    );
    assert_eq!(
        nth_fibonacci_memoized(7, &mut memoizer),
        BigUint::from(13u128)
    );
    assert_eq!(
        nth_fibonacci_memoized(8, &mut memoizer),
        BigUint::from(21u128)
    );
    assert_eq!(
        nth_fibonacci_memoized(9, &mut memoizer),
        BigUint::from(34u128)
    );
    assert_eq!(
        nth_fibonacci_memoized(10, &mut memoizer),
        BigUint::from(55u128)
    );
}

#[test]
fn should_generate_series_of_0_items() {
    assert_eq!(fibonacci_sequence(BigUint::from(0u128), 0), vec![]);
}

#[test]
fn should_generate_series_of_1_items() {
    assert_eq!(
        fibonacci_sequence(BigUint::from(0u128), 1),
        vec![BigUint::from(0u128)]
    );
}

#[test]
fn should_generate_series_of_10_items() {
    assert_eq!(
        fibonacci_sequence(BigUint::from(0u128), 10),
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
fn should_generate_series_with_given_starting_fibonacci() {
    assert_eq!(
        fibonacci_sequence(BigUint::from(0u128), 1),
        vec![BigUint::from(0u128)]
    );
    assert_eq!(
        fibonacci_sequence(BigUint::from(1u128), 1),
        vec![BigUint::from(1u128)]
    );
}

#[test]
fn should_generate_series_with_starting_fibonacci_1_if_given_2() {
    assert_eq!(
        fibonacci_sequence(BigUint::from(2u128), 2),
        vec![BigUint::from(1u128), BigUint::from(1u128)]
    );
}
