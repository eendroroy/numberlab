use std::cmp::max;
use std::ops::{Add, Div, Mul};

pub fn arithmetic_sequence(starting_number: f64, progression: f64, n: u128) -> Vec<f64> {
    let precision = max(
        starting_number.to_string().split(".").last().unwrap().len(),
        progression.to_string().split(".").last().unwrap().len(),
    ) as u32;
    let mut sequence = vec![starting_number];
    for _ in 1..n {
        let ith = sequence
            .last()
            .unwrap()
            .add(progression)
            .mul(10_i32.pow(precision) as f64)
            .round()
            .div(10_i32.pow(precision) as f64);

        sequence.push(ith);
    }
    sequence
}

#[cfg(test)]
mod arithmetic_test;
