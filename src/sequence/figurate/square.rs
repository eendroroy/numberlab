use num_bigint::BigUint;

pub fn nth_squire(n: u128) -> BigUint {
    BigUint::from(n).pow(2)
}

pub fn square_sequence(n: u128) -> Vec<BigUint> {
    let mut sequence = vec![];
    for i in 1..=n {
        sequence.push(nth_squire(i));
    }
    sequence
}

#[cfg(test)]
mod square_tests;
