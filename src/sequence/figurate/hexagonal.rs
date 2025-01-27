use num_bigint::BigUint;
use std::ops::{Div, Mul, Sub};

pub fn nth_hexagonal(n: u128) -> BigUint {
    BigUint::from(n)
        .mul(BigUint::from(2u128))
        .mul(
            BigUint::from(n)
                .mul(BigUint::from(2u128))
                .sub(BigUint::from(1u128)),
        )
        .div(BigUint::from(2u128))
}

pub fn hexagonal_sequence(n: u128) -> Vec<BigUint> {
    let mut sequence = vec![];
    for i in 1..=n {
        sequence.push(nth_hexagonal(i));
    }
    sequence
}
