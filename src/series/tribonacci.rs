use num_bigint::BigUint;

pub fn nth_tribonacci_memoized(n: usize, memoizer: &mut Vec<BigUint>) -> BigUint {
    if memoizer.len() == 0 {
        memoizer.push(BigUint::from(0u128));
        memoizer.push(BigUint::from(1u128));
        memoizer.push(BigUint::from(1u128));
    } else if memoizer.len() == 1 {
        memoizer.push(BigUint::from(1u128));
        memoizer.push(BigUint::from(1u128));
    } else if memoizer.len() == 2 {
        memoizer.push(BigUint::from(1u128));
    }

    if n < memoizer.len() {
        memoizer[n].clone()
    } else {
        let kth_tribonacci = nth_tribonacci_memoized(n - 3, memoizer);
        let lth_tribonacci = nth_tribonacci_memoized(n - 2, memoizer);
        let mth_tribonacci = nth_tribonacci_memoized(n - 1, memoizer);
        let nth_tribonacci = kth_tribonacci + lth_tribonacci + mth_tribonacci;
        memoizer.push(nth_tribonacci);
        memoizer[n].clone()
    }
}

pub fn tribonacci_sequence(n: usize) -> Vec<BigUint> {
    let mut sequence = vec![
        BigUint::from(0u128),
        BigUint::from(1u128),
        BigUint::from(1u128),
    ];
    for i in 0..n {
        nth_tribonacci_memoized(i, &mut sequence);
    }
    sequence[..n].to_vec()
}

#[cfg(test)]
mod tribonacci_tests;
