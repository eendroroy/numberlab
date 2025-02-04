use numberlab::algorithm::factorize::{factor_pairs, factors, gcd, lcm, prime_factors};

#[test]
fn should_generate_factor_pairs() {
    assert_eq!(factor_pairs(0), vec![]);
    assert_eq!(factor_pairs(1), vec![(1, 1)]);
    assert_eq!(factor_pairs(25), vec![(1, 25), (5, 5)]);
    assert_eq!(factor_pairs(28), vec![(1, 28), (2, 14), (4, 7)]);
    assert_eq!(
        factor_pairs(100),
        vec![(1, 100), (2, 50), (4, 25), (5, 20), (10, 10)]
    );
    assert_eq!(factor_pairs(101), vec![(1, 101)]);
    assert_eq!(factor_pairs(102), vec![(1, 102), (2, 51), (3, 34), (6, 17)]);
}

#[test]
fn should_generate_factors() {
    assert_eq!(factors(0), vec![]);
    assert_eq!(factors(1), vec![1]);
    assert_eq!(factors(25), vec![1, 5, 25]);
    assert_eq!(factors(28), vec![1, 2, 4, 7, 14, 28]);
    assert_eq!(factors(100), vec![1, 2, 4, 5, 10, 20, 25, 50, 100]);
    assert_eq!(factors(101), vec![1, 101]);
    assert_eq!(factors(102), vec![1, 2, 3, 6, 17, 34, 51, 102]);
}

#[test]
fn should_generate_prime_factors() {
    assert_eq!(prime_factors(0), vec![]);
    assert_eq!(prime_factors(1), vec![]);
    assert_eq!(prime_factors(25), vec![5, 5]);
    assert_eq!(prime_factors(28), vec![2, 2, 7]);
    assert_eq!(prime_factors(100), vec![2, 2, 5, 5]);
    assert_eq!(prime_factors(101), vec![101]);
    assert_eq!(prime_factors(102), vec![2, 3, 17]);
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
