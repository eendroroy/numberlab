use std::ops::{Div, Mul};

pub fn nth_triangular(n: u128) -> u128 {
    n.mul(n + 1).div(2)
}

pub fn triangular_sequence(n: usize) -> Vec<u128> {
    (0..n as u128).map(nth_triangular).collect()
}
