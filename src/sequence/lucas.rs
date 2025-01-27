use num_bigint::BigUint;

pub fn nth_lucas_memoized(n: usize, memoizer: &mut Vec<BigUint>) -> BigUint {
    if n < memoizer.len() {
        memoizer[n].clone()
    } else {
        let lth_lucas = nth_lucas_memoized(n - 1, memoizer);
        let mth_lucas = nth_lucas_memoized(n - 2, memoizer);
        let nth_lucas = lth_lucas + mth_lucas;
        memoizer.push(nth_lucas);
        memoizer[n].clone()
    }
}

pub fn lucas_sequence(n: usize) -> Vec<BigUint> {
    let mut sequence = vec![BigUint::from(2u128), BigUint::from(1u128)];
    for i in 0..n {
        nth_lucas_memoized(i, &mut sequence);
    }
    sequence[..n].to_vec()
}
