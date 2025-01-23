use super::*;

#[test]
fn test_nth_factorial_memoized() {
    let mut memoizer = vec![BigUint::from(1u128)];
    assert_eq!(BigUint::from(1u128), nth_factorial_memoized(0, &mut memoizer));
    assert_eq!(BigUint::from(1u128), nth_factorial_memoized(1, &mut memoizer));
    assert_eq!(BigUint::from(2u128), nth_factorial_memoized(2, &mut memoizer));
    assert_eq!(BigUint::from(6u128), nth_factorial_memoized(3, &mut memoizer));
    assert_eq!(BigUint::from(24u128), nth_factorial_memoized(4, &mut memoizer));
    assert_eq!(BigUint::from(120u128), nth_factorial_memoized(5, &mut memoizer));
    assert_eq!(BigUint::from(720u128), nth_factorial_memoized(6, &mut memoizer));
    assert_eq!(BigUint::from(5040u128), nth_factorial_memoized(7, &mut memoizer));
    assert_eq!(BigUint::from(40320u128), nth_factorial_memoized(8, &mut memoizer));
    assert_eq!(BigUint::from(362880u128), nth_factorial_memoized(9, &mut memoizer));
    assert_eq!(BigUint::from(3628800u128), nth_factorial_memoized(10, &mut memoizer));
}

#[test]
fn test_factorial_sequence() {
    assert_eq!(
        vec![
            BigUint::from(1u128),
            BigUint::from(1u128),
            BigUint::from(2u128),
            BigUint::from(6u128),
            BigUint::from(24u128),
            BigUint::from(120u128),
            BigUint::from(720u128),
            BigUint::from(5040u128),
            BigUint::from(40320u128),
            BigUint::from(362880u128),
        ],
        factorial_sequence(10)
    );
}
