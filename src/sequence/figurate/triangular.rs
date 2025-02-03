use num_bigint::BigUint;
use std::ops::{Div, Mul};

pub fn nth_triangular(n: u128) -> BigUint {
    BigUint::from(n)
        .mul(&BigUint::from(n + 1u128))
        .div(&BigUint::from(2u128))
}

pub fn triangular_sequence(n: u128) -> Vec<BigUint> {
    let mut sequence = vec![];
    (0..n).for_each(|i| sequence.push(nth_triangular(i)));
    sequence
}
