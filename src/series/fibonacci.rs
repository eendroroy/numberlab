use num_bigint::BigUint;

pub fn nth_fibonacci(n: usize) -> BigUint {
    if n == 0 {
        BigUint::from(0u128)
    } else if n == 1 {
        BigUint::from(1u128)
    } else {
        nth_fibonacci(n - 1) + nth_fibonacci(n - 2)
    }
}

pub fn nth_fibonacci_memoized(n: usize) -> BigUint {
    let mut memoizer = vec![BigUint::from(0u128), BigUint::from(1u128)];
    nth_fibonacci_with_memoizer(n, &mut memoizer)
}

pub fn nth_fibonacci_with_memoizer(n: usize, memoizer: &mut Vec<BigUint>) -> BigUint {
    if memoizer.len() > n {
        memoizer[n].clone()
    } else {
        let lth_fibonacci = nth_fibonacci_with_memoizer(n - 1, memoizer);
        let mth_fibonacci = nth_fibonacci_with_memoizer(n - 2, memoizer);
        let nth_fibonacci = lth_fibonacci + mth_fibonacci;
        memoizer.push(nth_fibonacci);
        memoizer[n].clone()
    }
}

pub fn fibonacci_sequence(n: usize) -> Vec<BigUint> {
    if n == 0 {
        vec![]
    } else if n == 1 {
        vec![BigUint::from(0u128)]
    } else if n == 2 {
        vec![BigUint::from(0u128), BigUint::from(1u128)]
    } else {
        let mut sequence = vec![BigUint::from(0u128), BigUint::from(1u128)];
        (2..n).for_each(|i| sequence.push(&sequence[i - 1] + &sequence[i - 2]));
        sequence
    }
}

#[cfg(test)]
mod fibonacci_tests;
