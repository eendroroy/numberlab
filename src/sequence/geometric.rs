pub fn nth_geometric(starting_number: f64, progression: f64, n: u64) -> f64 {
    starting_number * progression.powf(n as f64)
}

pub fn geometric_sequence(starting_number: f64, progression: f64, n: u64) -> Vec<f64> {
    let mut sequence = vec![starting_number];
    for _ in 1..n {
        let ith = nth_geometric(sequence.last().unwrap().clone(), progression, 1);

        sequence.push(ith);
    }
    sequence
}

pub fn geometric_sum(starting_number: f64, progression: f64, n: u64) -> f64 {
    if n < 2 {
        starting_number
    } else if progression == 1.0 {
        starting_number * n as f64
    } else {
        starting_number * ((progression.powf(n as f64) - 1f64) / (progression - 1f64))
    }
}

#[cfg(test)]
mod geometric_test;
