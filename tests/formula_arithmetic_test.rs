use numberlab::formula::arithmetic::{combination, factorial, gcd, lcm, permutation};

#[test]
#[should_panic(expected = "'n' (100) must be greater than or equal to 'r' (101)")]
fn should_panic_if_n_less_than_r_for_combination() {
    combination(100, 101);
}

#[test]
fn should_generate_combination() {
    assert_eq!(combination(1, 1), 1);
    assert_eq!(combination(2, 1), 2);
    assert_eq!(combination(2, 2), 1);
    assert_eq!(combination(3, 1), 3);
    assert_eq!(combination(3, 2), 3);
    assert_eq!(combination(3, 3), 1);
    assert_eq!(combination(4, 1), 4);
    assert_eq!(combination(4, 2), 6);
    assert_eq!(combination(4, 3), 4);
    assert_eq!(combination(4, 4), 1);
    assert_eq!(combination(5, 1), 5);
    assert_eq!(combination(5, 2), 10);
    assert_eq!(combination(5, 3), 10);
    assert_eq!(combination(5, 4), 5);
    assert_eq!(combination(5, 5), 1);
    assert_eq!(combination(6, 1), 6);
}


#[test]
fn should_calculate_nth_factorial() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(3), 6);
    assert_eq!(factorial(4), 24);
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(6), 720);
    assert_eq!(factorial(7), 5040);
    assert_eq!(factorial(8), 40320);
    assert_eq!(factorial(9), 362880);
    assert_eq!(factorial(10), 3628800);
}


#[test]
fn should_calculate_gcd() {
    assert_eq!(gcd(0, 0), 0);
    assert_eq!(gcd(0, 1), 1);
    assert_eq!(gcd(1, 0), 1);
    assert_eq!(gcd(1, 1), 1);
    assert_eq!(gcd(2, 1), 1);
    assert_eq!(gcd(1, 2), 1);
    assert_eq!(gcd(2, 2), 2);
    assert_eq!(gcd(2, 4), 2);
    assert_eq!(gcd(4, 2), 2);
    assert_eq!(gcd(4, 6), 2);
    assert_eq!(gcd(6, 4), 2);
    assert_eq!(gcd(3, 5), 1);
    assert_eq!(gcd(31, 57), 1);
    assert_eq!(gcd(2_u128.pow(2), 2_u128.pow(4)), 2_u128.pow(2));
    assert_eq!(gcd(2_u128.pow(5), 2_u128.pow(4)), 2_u128.pow(4));
    assert_eq!(gcd(2_u128.pow(5), 3_u128.pow(4)), 1);
}

#[test]
fn should_calculate_lcm() {
    assert_eq!(lcm(0, 0), 0);
    assert_eq!(lcm(0, 1), 0);
    assert_eq!(lcm(1, 0), 0);
    assert_eq!(lcm(1, 1), 1);
    assert_eq!(lcm(2, 1), 2);
    assert_eq!(lcm(1, 2), 2);
    assert_eq!(lcm(2, 2), 2);
    assert_eq!(lcm(2, 4), 4);
    assert_eq!(lcm(4, 2), 4);
    assert_eq!(lcm(4, 6), 12);
    assert_eq!(lcm(6, 4), 12);
    assert_eq!(lcm(3, 5), 15);
    assert_eq!(lcm(31, 57), 1767);
    assert_eq!(lcm(2_u128.pow(2), 2_u128.pow(4)), 2_u128.pow(4));
    assert_eq!(lcm(2_u128.pow(5), 2_u128.pow(4)), 2_u128.pow(5));
    assert_eq!(
        lcm(2_u128.pow(5), 3_u128.pow(4)),
        2_u128.pow(5) * 3_u128.pow(4)
    );
}

#[test]
#[should_panic(expected = "'n' (100) must be greater than or equal to 'r' (101)")]
fn should_panic_if_n_less_than_r_for_permutation() {
    permutation(100, 101);
}

#[test]
fn should_generate_permutation() {
    assert_eq!(permutation(1, 1), 1);
    assert_eq!(permutation(2, 1), 2);
    assert_eq!(permutation(2, 2), 2);
    assert_eq!(permutation(3, 1), 3);
    assert_eq!(permutation(3, 2), 6);
    assert_eq!(permutation(3, 3), 6);
    assert_eq!(permutation(4, 1), 4);
    assert_eq!(permutation(4, 2), 12);
    assert_eq!(permutation(4, 3), 24);
    assert_eq!(permutation(4, 4), 24);
    assert_eq!(permutation(5, 1), 5);
    assert_eq!(permutation(5, 2), 20);
    assert_eq!(permutation(5, 3), 60);
    assert_eq!(permutation(5, 4), 120);
    assert_eq!(permutation(5, 5), 120);
}
