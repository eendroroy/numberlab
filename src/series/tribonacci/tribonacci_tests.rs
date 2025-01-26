use super::*;

#[test]
fn should_generate_nth_tribonacci() {
    let mut memoizer = vec![
        BigUint::from(0u128),
        BigUint::from(1u128),
        BigUint::from(1u128),
    ];
    assert_eq!(
        nth_tribonacci_memoized(0, &mut memoizer),
        BigUint::from(0u128)
    );
    assert_eq!(
        nth_tribonacci_memoized(1, &mut memoizer),
        BigUint::from(1u128)
    );
    assert_eq!(
        nth_tribonacci_memoized(2, &mut memoizer),
        BigUint::from(1u128)
    );
    assert_eq!(
        nth_tribonacci_memoized(3, &mut memoizer),
        BigUint::from(2u128)
    );
    assert_eq!(
        nth_tribonacci_memoized(4, &mut memoizer),
        BigUint::from(4u128)
    );
    assert_eq!(
        nth_tribonacci_memoized(5, &mut memoizer),
        BigUint::from(7u128)
    );
    assert_eq!(
        nth_tribonacci_memoized(6, &mut memoizer),
        BigUint::from(13u128)
    );
    assert_eq!(
        nth_tribonacci_memoized(7, &mut memoizer),
        BigUint::from(24u128)
    );
    assert_eq!(
        nth_tribonacci_memoized(8, &mut memoizer),
        BigUint::from(44u128)
    );
    assert_eq!(
        nth_tribonacci_memoized(9, &mut memoizer),
        BigUint::from(81u128)
    );
    assert_eq!(
        nth_tribonacci_memoized(10, &mut memoizer),
        BigUint::from(149u128)
    );
}

#[test]
fn should_generate_series_of_0_items() {
    assert_eq!(tribonacci_sequence(0), vec![]);
}

#[test]
fn should_generate_series_of_1_items() {
    assert_eq!(tribonacci_sequence(1), vec![BigUint::from(0u128)]);
}

#[test]
fn should_generate_series_of_10_items() {
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
