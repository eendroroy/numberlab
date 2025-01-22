use num_bigint::BigUint;
use std::ops::{Div, Mul};

pub fn nth_triangle(n: u128) -> BigUint {
    BigUint::from(n)
        .mul(&BigUint::from(n + 1u128))
        .div(&BigUint::from(2u128))
}

pub fn triangle_sequence(n: u128) -> Vec<BigUint> {
    let mut sequence = vec![];
    for i in 1..=n {
        sequence.push(nth_triangle(i));
    }
    sequence
}

#[cfg(test)]
mod triangle_tests;
