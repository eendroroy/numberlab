use num_bigint::BigUint;
use std::ops::Mul;

pub fn nth_factorial(n: usize) -> BigUint {
    (1..=n)
        .map(|i| BigUint::from(i))
        .fold(BigUint::from(1u128), |acc, x| acc.mul(x))
}

pub fn nth_factorial_memoized(n: usize) -> BigUint {
    let mut memoizer = vec![BigUint::from(1u128)];
    nth_factorial_with_memoizer(n, &mut memoizer)
}

pub fn nth_factorial_with_memoizer(n: usize, memoizer: &mut Vec<BigUint>) -> BigUint {
    if n < memoizer.len() {
        memoizer[n].clone()
    } else {
        let mth_factorial = nth_factorial_with_memoizer(n - 1, memoizer);
        let nth_factorial = mth_factorial.mul(n);
        memoizer.push(nth_factorial);
        memoizer[n].clone()
    }
}

pub fn factorial_sequence(n: usize) -> Vec<BigUint> {
    if n == 0 {
        vec![]
    } else if n == 1 {
        vec![BigUint::from(1u128)]
    } else {
        let mut sequence = vec![BigUint::from(1u128)];
        (1..n).for_each(|i| sequence.push(&sequence[i - 1] * BigUint::from(i)));
        sequence[..n].to_vec()
    }
}

#[cfg(test)]
mod factorial_test;
