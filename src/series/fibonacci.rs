use num_bigint::BigUint;

pub fn nth_fibonacci_memoized(n: usize, memoizer: &mut Vec<BigUint>) -> BigUint {
    if n < memoizer.len() {
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
    let mut sequence = vec![first_fibonacci, BigUint::from(1u128)];
    for i in 0..n {
        nth_fibonacci_memoized(i, &mut sequence);
    }
    sequence
}
