use num_bigint::BigUint;
use std::ops::{Add, Div};

pub fn nth_lazy_caterer(n: u128) -> BigUint {
    BigUint::from(n)
        .pow(2)
        .add(BigUint::from(n))
        .add(BigUint::from(2u128))
        .div(BigUint::from(2u128))
}

pub fn lazy_caterer_sequence(n: u128) -> Vec<BigUint> {
    let mut sequence = vec![];
    for i in 0..n {
        sequence.push(nth_lazy_caterer(i));
    }
    sequence
}
