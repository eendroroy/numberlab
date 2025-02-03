pub fn nth_star(n: u128) -> u128 {
    6 * n * (n - 1) + 1
}

pub fn star_sequence(n: usize) -> Vec<u128> {
    match n {
        0 => vec![],
        _ => (1..=n as u128).map(nth_star).collect(),
    }
}
