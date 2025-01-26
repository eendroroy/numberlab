use num_bigint::BigUint;
use std::cmp::min;

pub fn nth_fibonacci_memoized(n: usize, memoizer: &mut Vec<BigUint>) -> BigUint {
    if memoizer.len() == 0 {
        memoizer.push(BigUint::from(0u128));
        memoizer.push(BigUint::from(1u128));
    } else if memoizer.len() == 1 {
        memoizer.push(BigUint::from(1u128));
    }

    if memoizer.len() > n {
        memoizer[n].clone()
    } else {
        let lth_fibonacci = nth_fibonacci_memoized(n - 1, memoizer);
        let mth_fibonacci = nth_fibonacci_memoized(n - 2, memoizer);
        let nth_fibonacci = lth_fibonacci + mth_fibonacci;
        memoizer.push(nth_fibonacci);
        memoizer[n].clone()
    }
}

pub fn fibonacci_sequence(first_fibonacci: BigUint, n: usize) -> Vec<BigUint> {
    let mut sequence = vec![
        min(first_fibonacci, BigUint::from(1u128)),
        BigUint::from(1u128),
    ];
    for i in 0..n {
        nth_fibonacci_memoized(i, &mut sequence);
    }
    sequence[..n].to_vec()
}

#[cfg(test)]
mod fibonacci_tests;
