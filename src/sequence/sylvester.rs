use num_bigint::BigUint;

pub fn nth_sylvester_memoized(n: usize, memoizer: &mut Vec<BigUint>) -> BigUint {
    if n < memoizer.len() {
        memoizer[n].clone()
    } else {
        let mth_sylvester = nth_sylvester_memoized(n - 1, memoizer);
        let nth_sylvester = mth_sylvester.pow(2u32) - mth_sylvester + BigUint::from(1u128);
        memoizer.push(nth_sylvester);
        memoizer[n].clone()
    }
}

pub fn sylvester_sequence(n: usize) -> Vec<BigUint> {
    let mut sequence = vec![BigUint::from(2u128)];
    for i in 0..n {
        nth_sylvester_memoized(i, &mut sequence);
    }
    sequence[..n].to_vec()
}
