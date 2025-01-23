use num_bigint::BigUint;
use std::ops::Mul;

pub fn nth_factorial_memoized(n: usize, memoizer: &mut Vec<BigUint>) -> BigUint {
    if n < memoizer.len() {
        memoizer[n].clone()
    } else {
        let mth_factorial = nth_factorial_memoized(n - 1, memoizer);
        let nth_factorial = mth_factorial.mul(n);
        memoizer.push(nth_factorial);
        memoizer[n].clone()
    }
}

pub fn factorial_sequence(n: usize) -> Vec<BigUint> {
    let mut sequence = vec![BigUint::from(1u128)];
    for i in 0..n {
        nth_factorial_memoized(i, &mut sequence);
    }
    sequence[..n].to_vec()
}

#[cfg(test)]
mod factorial_test;
