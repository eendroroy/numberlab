use std::cmp::max;
use std::ops::{Add, Div, Mul};

fn precision(starting_number: f64, progression: f64) -> u32 {
    max(
        starting_number.to_string().split(".").last().unwrap().len(),
        progression.to_string().split(".").last().unwrap().len(),
    ) as u32
}

pub fn nth_arithmetic(starting_number: f64, progression: f64, n: u128) -> f64 {
    let precision = precision(starting_number, progression);
    let nth = starting_number
        .add(progression.mul(n as f64))
        .mul(10_i32.pow(precision) as f64)
        .round()
        .div(10_i32.pow(precision) as f64);
    nth
}

pub fn arithmetic_sequence(starting_number: f64, progression: f64, n: u128) -> Vec<f64> {
    let mut sequence = vec![starting_number];
    for _ in 1..n {
        let ith = nth_arithmetic(sequence.last().unwrap().clone(), progression, 1);

        sequence.push(ith);
    }
    sequence
}

pub fn arithmetic_sum(starting_number: f64, progression: f64, n: u64) -> f64 {
    let precision = precision(starting_number, progression);
    (((2.0 * starting_number + ((n as f64) - 1.0) * progression) * (n as f64)) / 2.0)
        .mul(10_i32.pow(precision) as f64)
        .round()
        .div(10_i32.pow(precision) as f64)
}

#[cfg(test)]
mod arithmetic_test;
