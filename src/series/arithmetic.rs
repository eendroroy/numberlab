pub fn nth_arithmetic(starting_number: f64, progression: f64, n: u128) -> f64 {
    starting_number + (progression * n as f64)
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
    ((2.0 * starting_number + ((n as f64) - 1.0) * progression) * (n as f64)) / 2.0
}

#[cfg(test)]
mod arithmetic_test;
