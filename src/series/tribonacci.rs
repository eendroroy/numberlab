use num_bigint::BigUint;

pub fn nth_tribonacci(n: usize) -> BigUint {
    if n == 0 {
        BigUint::from(0u128)
    } else if n == 1 {
        BigUint::from(1u128)
    } else if n == 2 {
        BigUint::from(1u128)
    } else {
        nth_tribonacci(n - 3) + nth_tribonacci(n - 2) + nth_tribonacci(n - 1)
    }
}

pub fn nth_tribonacci_memoized(n: usize) -> BigUint {
    let mut memoizer = vec![
        BigUint::from(0u128),
        BigUint::from(1u128),
        BigUint::from(1u128),
    ];
    nth_tribonacci_with_memoizer(n, &mut memoizer)
}

pub fn nth_tribonacci_with_memoizer(n: usize, memoizer: &mut Vec<BigUint>) -> BigUint {
    if memoizer.len() > n {
        memoizer[n].clone()
    } else {
        let kth_tribonacci = nth_tribonacci_with_memoizer(n - 3, memoizer);
        let lth_tribonacci = nth_tribonacci_with_memoizer(n - 2, memoizer);
        let mth_tribonacci = nth_tribonacci_with_memoizer(n - 1, memoizer);
        let nth_tribonacci = kth_tribonacci + lth_tribonacci + mth_tribonacci;
        memoizer.push(nth_tribonacci);
        memoizer[n].clone()
    }
}

pub fn tribonacci_sequence(n: usize) -> Vec<BigUint> {
    if n == 0 {
        vec![]
    } else if n == 1 {
        vec![BigUint::from(0u128)]
    } else if n == 2 {
        vec![BigUint::from(0u128), BigUint::from(1u128)]
    } else if n == 3 {
        vec![
            BigUint::from(0u128),
            BigUint::from(1u128),
            BigUint::from(1u128),
        ]
    } else {
        let mut sequence = vec![
            BigUint::from(0u128),
            BigUint::from(1u128),
            BigUint::from(1u128),
        ];
        (3..n).for_each(|i| sequence.push(&sequence[i - 1] + &sequence[i - 2] + &sequence[i - 3]));
        sequence
    }
}

#[cfg(test)]
mod tribonacci_tests;
